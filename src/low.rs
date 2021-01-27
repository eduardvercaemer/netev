use std::net::UdpSocket;
use std::thread;
use std::sync::mpsc;

/// # Slave event listener
/// 
/// its main purpose is binding to a udp port and listening for
/// connections, it then handles this connections to populate a queue
/// of received packets (bytes + addr)
/// 
/// it interacts with the master thread via two channels, whenever it
/// gets a requests, it sends back a response accordingly
/// 
/// # Panics
/// TODO:
/// - if it does not answer back to a request, the master thread will
///   be locked forever
struct EvSlave {
    /// Queue of packets
    queue: Vec<Packet>,
    /// Socket the slave binds to
    sock: UdpSocket,

    /// Send messages to master
    tx: mpsc::Sender<Msg>,
    /// Receive messages from master
    rx: mpsc::Receiver<Msg>,
}

/// # Master event listener
/// 
/// owns a slave thread that listens for network packets, it then has the
/// ability to pop packets from the queue in the slave thread whenever it
/// wants
pub struct EvMaster {
    /// Send messages to slave
    tx: mpsc::Sender<Msg>,
    /// Receive messages from slave
    rx: mpsc::Receiver<Msg>,
}

/// Msgs between threads
enum Msg {
    /// Request to slave
    Req,

    /// Slave returns a packet
    Packet(Packet),
    /// Slave returns empty
    Empty,

    /// Any err msg
    Err,
}

/// Represents a single packet received by a slave thread,
/// which is kept in the queue until requested by the master
#[derive(Debug)]
pub struct Packet {
    pub bytes: Vec<u8>,
    pub addr: String,
}

impl EvMaster {
    /// Create new master thread with corresponding slave thread
    /// listening on given port
    pub fn new(port: &str) -> Self {
        const MAX_PACKET_SIZE: usize = 256;

        // master to slave
        let (tx0, rx0) = mpsc::channel();
        // slave to master
        let (tx1, rx1) = mpsc::channel();
        
        // create the socket
        let sock = UdpSocket::bind(format!("localhost:{}", port)).unwrap();
        sock.set_nonblocking(true).unwrap();

        // create the slave
        let mut slave = EvSlave {
            tx: tx1,
            rx: rx0,
            queue: vec![],
            sock,
        };

        // move slave into own thread
        thread::spawn(move || loop {
            // see if there are requests
            if let Ok(msg) = slave.rx.try_recv() {
                //println!("slave: got request from master . . .");
                // handle the request if there is one
                match msg {
                    // master asked for next event in queue
                    Msg::Req => {
                        if slave.queue.len() == 0 {
                            // queue is empty
                            slave.tx.send(Msg::Empty).unwrap();
                        } else {
                            // dispatch next event in queue
                            let top = slave.queue.pop().unwrap();
                            slave.tx.send(Msg::Packet(top)).unwrap();
                        }
                    }
                    // some unexpected error
                    _ => {
                        slave.tx.send(Msg::Err).unwrap();
                    }
                }
            }

            // slave logic . . .
            // get packets of up to MAX_PACKET_SIZE bytes
            let mut buf: Vec<u8> = Vec::with_capacity(MAX_PACKET_SIZE);
            buf.resize(MAX_PACKET_SIZE, 0);

            // attempt to receive a packet
            if let Ok((bytes, addr)) = slave.sock.recv_from(&mut buf[..]) {
                println!("slave: receive {} bytes from {}", bytes, addr);
                buf.resize(bytes, 0);
                slave.queue.push(Packet {
                    bytes: buf,
                    addr: format!("{}", addr),
                });
            }
        });

        // return the new master
        Self {
            tx: tx0,
            rx: rx1,
        }
    }

    /// Ask for next packet in owned slave thread
    pub fn next(&mut self) -> Option<Packet> {
        // request from slave thread
        self.tx.send(Msg::Req).unwrap();
        // wait for response
        match self.rx.recv().unwrap() {
            // if slave returns a packet, return that
            Msg::Packet(e) => {
                Some(e)
            }
            // otherwise we return None
            _ => {
                None
            }
        }
    }
}

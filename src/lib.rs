use std::net::UdpSocket;
use std::thread;
use std::sync::mpsc;
use std::error::Error;
use std::marker::PhantomData;
use serde::Serialize;
use serde::de::{DeserializeOwned};

/// # Netev Sender
/// 
/// # Function
/// Binds to a UDP port and can send messages to a netev [`Receiver`]
/// where [`T`] is the type of the events we want to send.
pub struct Sender<T> {
    sock: UdpSocket,
    phantom: PhantomData<T>,
}

/// # Netev Receiver
/// 
/// # Function
/// Owns a slave thread that listens for network packets, it then has the
/// ability to pop packets from the queue in the slave thread whenever it
/// wants.
/// 
/// The type [`T`] is the type of the events we will receive.
pub struct Receiver<T> {
    /// Send messages to slave
    tx: mpsc::Sender<Msg>,
    /// Receive messages from slave
    rx: mpsc::Receiver<Msg>,
    phantom: PhantomData<T>,
}

/// # Netev Receiver Slave
/// Its main purpose is binding to a udp port and listening for
/// connections, it then handles this connections to populate a queue
/// of received packets (bytes + addr)
/// 
/// It interacts with the master thread via two channels, whenever it
/// gets a requests, it sends back a response accordingly
/// 
/// # Panics
/// TODO:
/// - if it does not answer back to a request, the master thread will
///   be locked forever
/// - Add support for multiple UDP ports or even TCP connections
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

/// Msgs between threads
#[derive(Debug)]
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

impl<T> Receiver<T>
where
    T: DeserializeOwned,
{
    /// Create new  netev [`Receiver`] with its own slave listening thread
    pub fn bind(port: &str) -> Result<Self, Box<dyn Error>> {
        // XXX: Change this !!!
        const MAX_PACKET_SIZE: usize = 256;

        // master to slave
        let (tx0, rx0) = mpsc::channel();
        // slave to master
        let (tx1, rx1) = mpsc::channel();
        
        // create the socket
        let sock = UdpSocket::bind(port)?;
        sock.set_nonblocking(true)?;

        // create the slave
        let mut slave = EvSlave {
            tx: tx1,
            rx: rx0,
            queue: vec![],
            sock,
        };

        // move slave into own thread
        // TODO: handle panics in here !!!
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
        Ok(Self {
            tx: tx0,
            rx: rx1,
            phantom: PhantomData,
        })
    }

    /// Ask for next packet in owned slave thread
    /// XXX: panics in here !!!
    pub fn try_recv(&mut self) -> Option<T> {
        // request from slave thread
        self.tx.send(Msg::Req).unwrap();
        // wait for response
        // XXX: will lock if slave doesn't answer back
        match self.rx.recv().unwrap() {
            // if slave returns a packet, return the deserialization
            Msg::Packet(e) => {
                Some(bincode::deserialize::<T>(e.bytes.as_slice()).unwrap())
            }
            // otherwise we return None
            _ => {
                None
            }
        }
    }
}

impl<T> Sender<T>
where
    T: Serialize,
{
    /// Attempt to bind a netev sender and connect to a destination.
    pub fn bind(port: &str, dest: &str) -> Result<Self, Box<dyn Error>> {
        let sock = UdpSocket::bind(port)?;
        sock.connect(dest)?;
        Ok(Self { sock, phantom: PhantomData })
    }

    /// Use a pusher to send a serialize object to its destination
    pub fn push(&mut self, e: &T) -> Result<(), Box<dyn Error>> {
        let buf = bincode::serialize(&e)?;
        self.sock.send(buf.as_slice())?;
        Ok(())
    }

}
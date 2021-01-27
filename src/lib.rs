use std::io::prelude::*;
use std::net::UdpSocket;
use std::thread;
use std::sync::mpsc;

/// Slave thread
/// 
/// is main purpose is binding to a udp port and listening for
/// connections, it then handles this connections to populate a queue
/// of events
/// 
/// it interacts with the master thread via two channels, whenever it
/// gets a requests, it sends back a response accordingly
/// 
/// # Panics
/// TODO:
/// - if it does not answer back to a request, the master thread will
///   be locked forever
struct EvSlave {
    /// Queue of events
    queue: Vec<Event>,
    /// Socket the slave binds to
    sock: UdpSocket,

    /// Send messages to master
    tx: mpsc::Sender<Msg>,
    /// Receive messages from master
    rx: mpsc::Receiver<Msg>,
}

/// Master thread
/// 
/// Its main purpose is owning and interacting with its slave thread,
/// as well as serving as an iterator for events, whenever asked for
/// an event, it requests it from the slave using the channels they have.
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

    /// Slave returns an event
    Event(Event),
    /// Slave returns empty
    Empty,

    /// Any err msg
    Err,
}

/// Posible events
#[derive(Debug)]
pub enum Event {
    Dbg,
}

impl EvMaster {
    pub fn new(port: &str) -> Self {
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
                            slave.tx.send(Msg::Event(top)).unwrap();
                        }
                    }
                    // some unexpected error
                    _ => {
                        slave.tx.send(Msg::Err).unwrap();
                    }
                }
            }

            // slave logic . . .
            let mut buf = [0; 64];

            // attempt to receive a packet
            if let Ok((bytes, addr)) = slave.sock.recv_from(&mut buf) {
                println!("slave: receive {} bytes from {}", bytes, addr);
                slave.queue.push(Event::Dbg);
            }
        });

        // return the new master
        Self {
            tx: tx0,
            rx: rx1,
        }
    }

    pub fn next(&mut self) -> Option<Event> {
        // request from slave thread
        self.tx.send(Msg::Req).unwrap();
        // wait for response
        match self.rx.recv().unwrap() {
            Msg::Event(e) => {
                Some(e)
            }
            _ => {
                None
            }
        }
    }
}


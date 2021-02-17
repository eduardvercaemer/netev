extern crate serde;
extern crate bincode;
use std::net::UdpSocket;
use std::error::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;
mod low;

/* PUBLIC API FOR NETWORK EVENT CREATION AND QUEUE-ING */

/// Manages a queue of UDP packets
pub struct Popper {
    queue: low::EvMaster,
}

/// Creates network events a `Popper` can listen to
pub struct Pusher{
    sock: UdpSocket,
}

/// Create packets a `Popper` can listen to
//pub struct Pusher<T>;

impl Popper {
    /// Create new `Popper` listening on given port
    pub fn bind(port: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            queue: low::EvMaster::new(port)?,
        })
    }

    /// Pop a packet from the queue and attempt to
    /// deserialize into `T`
    pub fn pop<T>(&mut self) -> Option<T>
    where
        T: DeserializeOwned
    {
        match self.queue.next() {
            Some(data) => {
                // we popped a packet from the queue
                match bincode::deserialize(data.bytes.as_slice()) {
                    Ok(e) => Some(e),
                    Err(_) => None,
                }
            }
            _ => {
                None
            }
        }
    }
}

impl Pusher {
    /// Attempt to create a pusher bound to a port, and connected
    /// to a destination
    pub fn bind(port: &str, dest: &str) -> Result<Self, Box<dyn Error>> {
        let sock = UdpSocket::bind(port)?;
        sock.connect(dest)?;
        Ok(Self { sock })
    }

    /// Use a pusher to send a serialize object to its destination
    pub fn push<T>(&mut self, e: &T) -> Result<(), Box<dyn Error>>
    where
        T: Serialize,
    {
        let buf = bincode::serialize(&e)?;
        self.sock.send(buf.as_slice())?;
        Ok(())
    }
}
extern crate serde;
extern crate serde_json;
use std::net::UdpSocket;
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
    pub fn bind(port: &str) -> Self {
        Self {
            queue: low::EvMaster::new(port),
        }
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
                let de = serde_json::from_slice(data.bytes.as_slice());
                let de = de.unwrap();
                Some(de)
            }
            _ => {
                None
            }
        }
    }
}

impl Pusher {
    pub fn bind(port: &str, dest: &str) -> Self {
        let sock = UdpSocket::bind(format!("localhost:{}", port)).unwrap();
        sock.connect(dest).unwrap();
        Self { sock }
    }

    pub fn push<T>(&mut self, e: &T)
    where
        T: Serialize,
    {
        let buf = serde_json::to_vec(&e).unwrap();
        self.sock.send(buf.as_slice()).unwrap();
    }
}
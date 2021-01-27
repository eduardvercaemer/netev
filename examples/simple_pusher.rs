extern crate netev;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
mod simple_msg;
use simple_msg::Msg;

fn main() {
    /*
       we can use this simple program to send dummy packets
       to our program to test the events it produces
    */
    let mut pusher = netev::Pusher::bind("35423", "localhost:8000");
    /* send some quick messages */
    let msg = Msg {
        name: "Eduard".to_owned(),
        age: 19,
    };
    pusher.push(&msg);
}
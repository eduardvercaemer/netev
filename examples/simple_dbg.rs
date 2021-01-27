use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

fn main() {
    /*
       we can use this simple program to send dummy packets
       to our program to test the events it produces
    */
    let sock = UdpSocket::bind("localhost:34254").unwrap();
    sock.connect("localhost:8000").unwrap();
    /* send some quick messages */
    for _ in 0..10 {
        sock.send(&[0, 2, 3]).unwrap();
        thread::sleep(Duration::from_millis(50));
    }
}
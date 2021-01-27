use std::net::UdpSocket;

fn main() {
    let sock = UdpSocket::bind("localhost:34254").unwrap();
    sock.connect("localhost:8000").unwrap();
    sock.send(&[0, 2, 3]).unwrap();
}
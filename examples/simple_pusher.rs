extern crate netev;
mod simple_msg;
use simple_msg::Msg;
use text_io::read;

fn main() {
    /*
       we can use this simple program to send dummy packets
       to our program to test the events it produces
    */
    println!("Connecting to localhost:8000 . . .");
    let mut pusher = netev::Pusher::bind("localhost:35423", "localhost:8000")
        .expect("Failed to bind event pusher !");
    /* send some quick messages */
    loop {
        println!("Create a message:");
        println!("name:");
        let name: String = read!();
        println!("age:");
        let age: u8 = read!();
        let msg = Msg {
            age,
            name,
        };
        println!("Sending message . . .");
        pusher.push(&msg)
            .expect("Failed to push event !");
    }
}
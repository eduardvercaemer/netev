extern crate netev;

mod msg;
use msg::Msg;

fn main() {
    /* usage example */

    /*
       we create a network event listener, binded to a UDP port,
       8000 in this case

       it spawns a slave thread that populates an event queue based
       on packets it receives at the port
    */
    println!("Listening on port 8000 . . .");
    let mut receiver = netev::Receiver::<Msg>::bind("localhost:8000")
        .expect("Failed to bind event listener");

    /* game loop */
    loop {
        /*
           we can then use the `next` method in the master thread, to
           ask the queue for the next event, yielding a Some(e) with
           the event if there is one (popping it from the queue), or
           None if there are none at the moment
        */
        while let Some(msg) = receiver.try_recv() {
            println!("Main thread popped a message !");
            dbg!(msg);
        }
    }
}
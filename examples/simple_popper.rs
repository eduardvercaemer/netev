extern crate netev;

mod simple_msg;
use simple_msg::Msg;

fn main() {
    /* usage example */

    /*
       we create a network event listener, binded to a UDP port,
       8000 in this case

       it spawns a slave thread that populates an event queue based
       on packets it receives in the port
    */
    let mut popper = netev::Popper::bind("8000");

    /* game loop */
    loop {
        /*
           we can then use the `next` method in the master thread, to
           ask the queue for the next event, yielding a Some(e) with
           the event if there is one (popping it from the queue), or
           None if there are none at the moment
        */

        if let Some(msg) = popper.pop::<Msg>() {
            println!("we got a msg!\n{:?}", msg);
        }
    }
}
extern crate netev;

fn main() {
    /* usage example */

    /* create new poller at given port */
    let mut poller = netev::EvMaster::new("8000");

    /* game loop */
    loop {
        /* get a single event from poller */
        if let Some(e) = poller.next() {
            println!("got event: {:?}", e);
        }
    }
}
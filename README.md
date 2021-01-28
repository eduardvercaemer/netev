# netev

Simple UDP message queue system for rust.

#### Usage

Listening for UDP messages:
```rust
use netev::Popper;

let mut popper = Popper::bind("8000");

// will not block if there are no messages
// otherwise returns the next message in the queue
if let Some(msg) = popper.pop::<Msg>() {
    // we have a msg ...
}
```

Creating UDP messages:
```rust
use netev::Pusher;

let mut pusher = Pusher::bind("354254", "localhost:8000");
let msg = Msg {
  name: "A name",
  age: 16,
  ...
}
pusher.push(&msg);
```

#### TODO

- [ ] TCP (?)
- [ ] handle clients instead of independent messages

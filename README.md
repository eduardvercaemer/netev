# netev

Simple UDP message queue system for rust.

#### Usage

Listening for UDP messages:
```rust
use netev::Receiver;

// `Msg` is the type of the messages we will receive
let mut rx = Receiver::<Msg>::bind("localhost:8000")?;

// Will not block if there are no messages
// otherwise returns the next message in the queue
if let Some(msg) = rx.try_recv() {
    // we have a msg ...
}
```

Creating UDP messages:
```rust
use netev::Sender;

let mut tx = Sender::<Msg>::bind("localhost:354254", "localhost:8000")?;
let msg = Msg {
  name: "A name",
  age: 16,
  ...
}
tx.push(&msg)?;
```

Most times type inference allows us to omit the turbofish syntax.
```rust
use netev::Sender;

let mut tx = Sender::bind("localhost:354254", "localhost:8000")?;
let msg = Msg {
  name: "A name",
  age: 16,
  ...
}
tx.push(&msg)?;
```

#### TODO

- [ ] Implement Receiver as iterator (?)
- [ ] TCP (?)
- [ ] handle clients instead of independent messages

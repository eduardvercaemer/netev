extern crate serde;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Msg {
    pub age: u8,
    pub name: String,
}

pub fn main() {
    panic!();
}
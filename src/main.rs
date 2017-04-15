extern crate serde;
extern crate serde_json;
extern crate ssh2;

#[macro_use]
extern crate serde_derive;

pub mod constants;
pub mod information;
mod provider;

fn main() {
    println!("Hello, world!");
}

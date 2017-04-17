extern crate serde;
extern crate serde_json;
extern crate ssh2;

#[macro_use]
extern crate serde_derive;

pub mod constants;
pub mod information;
pub mod error;
mod provider;

#[cfg(test)]
mod test_helpers;

fn main() {
    println!("Hello, world!");
}

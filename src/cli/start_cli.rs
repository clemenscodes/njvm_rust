extern crate lib;
use std::env;

pub fn start_cli() {
    println!("{:?}", env::args())
}
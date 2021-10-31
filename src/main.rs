extern crate lib;
pub mod cli;
pub use cli::*;
use structopt::StructOpt;

fn main() {
    Njvm::start(Njvm::from_args())
}
pub mod njvm;
pub use njvm::*;
pub mod cpu;
pub use cpu::*;
pub mod memory;
pub use memory::*;
use std::env;
use std::io::{stdin, stdout, BufRead, Write};

type R = Box<dyn BufRead>;
type W = Box<dyn Write>;
pub type Bytecode = u32;
pub const VERSION: u32 = 2;
pub const MAXITEMS: u32 = 10000;

fn main() {
    let stdin = stdin();
    let input = stdin.lock();
    let output = stdout();
    let mut vm = NinjaVM::new(input, output);
    match env::args().len() {
        1 => NinjaVM::<R, W>::no_arg(),
        2..=3 => vm.check_arg(&env::args().nth(1).expect("Failed to parse argument")),
        _ => NinjaVM::<R, W>::kill(),
    }
}

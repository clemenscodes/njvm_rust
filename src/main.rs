pub mod njvm;
pub use njvm::*;
pub mod cpu;
pub use cpu::*;
pub mod memory;
pub use memory::*;
use std::env;
use std::io::{stdin, stdout, BufRead, Write};

pub type Bytecode = u32;
pub const VERSION: u32 = 2;
pub const MAXITEMS: u32 = 10000;

fn main() {
    let stdin = stdin();
    let mut vm = NinjaVM::new(stdin.lock(), stdout());
    match env::args().len() {
        1 => NinjaVM::<Box<dyn BufRead>, Box<dyn Write>>::no_arg(),
        2 => vm.check_arg(&env::args().nth(1).expect("Failed to parse argument")),
        _ => NinjaVM::<Box<dyn BufRead>, Box<dyn Write>>::kill(),
    }
}

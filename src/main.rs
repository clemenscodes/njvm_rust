pub mod njvm;
pub use njvm::*;
pub mod cpu;
pub use cpu::*;
pub mod memory;
pub use memory::*;
use std::env;

pub const VERSION: u32 = 2;

pub type Bytecode = u32;
pub const MAXITEMS: u32 = 10000;

fn main() {
    let mut vm = NinjaVM::default();
    match env::args().len() {
        1 => NinjaVM::no_arg(),
        2 => vm.check_arg(&env::args().nth(1).expect("Failed to parse argument")),
        _ => NinjaVM::kill(),
    }
}

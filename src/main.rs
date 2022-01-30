use crate::{NinjaVM, ProgramMemory};
pub mod njvm;
pub use njvm::*;
use std::env;
use std::process::exit;

fn main() {
    let args = env::args().skip(1);
    for arg in args {
        if arg == "--help" {
            println!("usage: ./njvm [option] [option] ...");
            println!("  --prog1          select program 1 to execute");
            println!("  --prog2          select program 2 to execute");
            println!("  --prog3          select program 3 to execute");
            println!("  --version        show version and exit");
            println!("  --help           show this help and exit");
            exit(0);
        }
        if arg == "--version" {
            println!("Ninja Virtual Machine version 1 (compiled Sep 23 2015, 10:36:52)");
            exit(0);
        }
        if arg == "--prog1" {
            let mut vm = NinjaVM::init();
            ProgramMemory::prog1(&mut vm);
        }
        if arg == "--prog2" {
            let mut vm = NinjaVM::init();
            ProgramMemory::prog2(&mut vm);
        }
        if arg == "--prog3" {
            let mut vm = NinjaVM::init();
            ProgramMemory::prog3(&mut vm);
        }
        println!("unknown command line argument '{arg}', try './njvm --help'");
        exit(1);
    }
}

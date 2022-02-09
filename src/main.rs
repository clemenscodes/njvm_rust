use std::env::args;
use std::io::{BufRead, Write};
pub mod njvm;
pub use njvm::*;
pub mod cpu;
pub use cpu::*;
pub mod memory;
pub use memory::*;
pub mod utils;
pub use utils::*;

fn main() {
    let mut vm = NinjaVM::default();
    match args().len() {
        1 => fatal_error("Error: no code file specified"),
        2 => {
            let arg = &args().nth(1).unwrap();
            check_arg(&mut vm, arg)
        }
        3 => {
            let bin = &args().nth(1).unwrap();
            let debug_flag = &args().nth(2).unwrap();
            check_args(&mut vm, bin, debug_flag)
        }
        _ => kill(),
    }
}

fn check_arg<R, W>(vm: &mut NinjaVM<R, W>, arg: &str)
where
    R: BufRead,
    W: Write,
{
    match arg {
        "--help" => help(),
        "--version" => version(),
        _ => vm.execute_binary(arg),
    }
}

fn check_args<R, W>(vm: &mut NinjaVM<R, W>, bin: &str, debug_flag: &str)
where
    R: BufRead,
    W: Write,
{
    match debug_flag {
        "--debug" => vm.debug_binary(bin),
        _ => unknown_arg(debug_flag),
    }
}

pub fn help() {
    println!("usage: ./njvm [options] <code file>");
    println!("  --version        show version and exit");
    println!("  --help           show this help and exit");
}

pub fn version() {
    println!("Ninja Virtual Machine version {} (compiled Sep 23 2015, 10:36:52)", VERSION);
}

fn kill() {
    help();
    #[cfg(not(test))]
    std::process::exit(1);
    #[cfg(test)]
    panic!();
}

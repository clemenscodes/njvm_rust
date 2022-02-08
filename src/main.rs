pub mod njvm;
pub use njvm::*;
pub mod cpu;
pub use cpu::*;
pub mod memory;
pub use memory::*;
use std::env;
use std::io::{stdin, stdout, BufRead, Write};
use std::process::exit;

pub type Bytecode = u32;
pub const VERSION: u32 = 2;
pub const MAXITEMS: u32 = 10000;

fn main() {
    let stdin = stdin();
    let input = stdin.lock();
    let output = stdout();
    match env::args().len() {
        1 => no_arg(),
        2 => {
            let arg = &env::args().nth(1).unwrap();
            let mut vm = NinjaVM::new(input, output);
            check_arg(&mut vm, arg)
        }
        3 => {
            let bin = &env::args().nth(1).unwrap();
            let debug_flag = &env::args().nth(2).unwrap();
            let mut vm = NinjaVM::new(input, output);
            check_args(&mut vm, bin, debug_flag)
        }
        _ => kill(),
    }
}

pub fn check_arg<R, W>(vm: &mut NinjaVM<R, W>, arg: &str)
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

pub fn check_args<R, W>(vm: &mut NinjaVM<R, W>, bin: &str, debug_flag: &str)
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
    println!("Ninja Virtual Machine version {VERSION} (compiled Sep 23 2015, 10:36:52)");
}

pub fn init() {
    println!("Ninja Virtual Machine started");
}

pub fn no_arg() {
    eprintln!("Error: no code file specified");
    exit(1)
}

pub fn unknown_arg(arg: &str) {
    eprintln!("unknown command line argument '{arg}', try './njvm --help'");
    exit(1);
}

pub fn kill() {
    help();
    exit(1)
}

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

fn main() {
    let stdin = stdin();
    let input = stdin.lock();
    let output = stdout();
    let mut vm = NinjaVM::new(input, output);
    match env::args().len() {
        1 => fatal_error("Error: no code file specified"),
        2 => {
            let arg = &env::args().nth(1).unwrap();
            check_arg(&mut vm, arg)
        }
        3 => {
            let bin = &env::args().nth(1).unwrap();
            let debug_flag = &env::args().nth(2).unwrap();
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

pub fn unknown_arg(arg: &str) {
    eprintln!("unknown command line argument '{arg}', try './njvm --help'");
    #[cfg(not(test))]
    std::process::exit(1);
    #[cfg(test)]
    panic!("unknown command line argument '{arg}', try './njvm --help'");
}

pub fn kill() {
    help();
    #[cfg(not(test))]
    std::process::exit(1);
    #[cfg(test)]
    panic!();
}

pub fn fatal_error(error: &str) -> ! {
    eprintln!("{error}");
    #[cfg(not(test))]
    std::process::exit(1);
    #[cfg(test)]
    panic!("{error}");
}

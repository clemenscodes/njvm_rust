extern crate lib;
use std::env;
use std::process::exit;

pub fn print_help() {
    println!("usage: ./njvm [option] [option] ...");
    println!("  --version        show version and exit");
    println!("  --help           show this help and exit");
    exit(0);
}

pub fn print_version() {
    println!("Ninja Virtual Machine version 0 (compiled Sep 23 2015, 10:36:52)");
    exit(0);
}

pub fn print_err(arg: &str) {
    println!("unknown command line argument '{arg}', try './njvm --help'");
    exit(0);
}

pub fn start_cli() {
    let args = env::args().skip(1);
    for arg in args {
        if arg == "--help" {
            print_help()
        }
        if arg == "--version" {
            print_version()
        }
        print_err(&arg)
    }
    println!("Ninja Virtual Machine started");
    println!("Ninja Virtual Machine stopped");
}

use std::env;
use std::process::exit;
// use crate::Stack;

pub fn start_njvm() {
    if env::args().count() < 2 {
        println!("Ninja Virtual Machine started");
        // let mut stack: Stack = Stack::new();
        println!("Ninja Virtual Machine stopped");
    }
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
            println!("Ninja Virtual Machine version 0 (compiled Sep 23 2015, 10:36:52)");
            exit(0);
        }
        if arg == "--prog1" {
            println!("prog1 selected");
            exit(0);
        }
        if arg == "--prog2" {
            println!("prog2 selected");
            exit(0);
        }
        if arg == "--prog3" {
            println!("prog3 selected");
            exit(0);
        }
        println!("unknown command line argument '{arg}', try './njvm --help'");
        exit(1);
    }
}

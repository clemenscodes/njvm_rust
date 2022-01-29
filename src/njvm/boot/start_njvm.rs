use std::env;
use std::process::exit;
use crate::{Worker, Opcode};

pub fn start_njvm() {
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
            let (mut stack, mut program_memory) = Worker::init();
            program_memory.register_instruction(&Opcode::Pushc, 3);
            program_memory.register_instruction(&Opcode::Pushc, 4);
            program_memory.register_instruction(&Opcode::Add, 0);
            program_memory.register_instruction(&Opcode::Pushc, 10);
            program_memory.register_instruction(&Opcode::Pushc, 6);
            program_memory.register_instruction(&Opcode::Sub, 0);
            program_memory.register_instruction(&Opcode::Mul, 0);
            program_memory.register_instruction(&Opcode::Wrint, 0);
            program_memory.register_instruction(&Opcode::Pushc, 10);
            program_memory.register_instruction(&Opcode::Wrchr, 0);
            program_memory.register_instruction(&Opcode::Halt, 0);
            program_memory.print();
            Worker::work(&mut program_memory, &mut stack);
        }
        if arg == "--prog2" {
            let (mut stack, mut program_memory) = Worker::init();
            Worker::work(&mut program_memory, &mut stack);
        }
        if arg == "--prog3" {
            let (mut stack, mut program_memory) = Worker::init();
            Worker::work(&mut program_memory, &mut stack);
        }
        println!("unknown command line argument '{arg}', try './njvm --help'");
        exit(1);
    }
}

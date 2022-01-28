use std::env;
use std::process::exit;

macro_rules! immediate {
    ($e:expr) => {
        ($e) & 0x00FFFFFF
    };
}

macro_rules! sign_extend {
    ($e:expr) => {
        if ((($e) & 0x00800000) == 1) {
            ($e) | 0xFF000000
        } else {
            ($e)
        }
    };
}

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Halt = 0,
    Pushc = 1,
    Add = 2,
    Sub = 3,
    Mul = 4,
    Div = 5,
    Mod = 6,
    Rdint = 7,
    Wrint = 8,
    Rdchr = 9,
    Wrchr = 10,
}

impl Opcode {
    pub fn match_opcode(code: u8) -> Opcode {
        let opcode: Opcode;
        match code {
            0 => opcode = Opcode::Halt,
            1 => opcode = Opcode::Pushc,
            2 => opcode = Opcode::Add,
            3 => opcode = Opcode::Sub,
            4 => opcode = Opcode::Mul,
            5 => opcode = Opcode::Div,
            6 => opcode = Opcode::Mod,
            7 => opcode = Opcode::Rdint,
            8 => opcode = Opcode::Wrint,
            9 => opcode = Opcode::Rdchr,
            10 => opcode = Opcode::Wrchr,
            _ => panic!("Invalid opcode"),
        }
        opcode
    }
}

#[derive(Debug)]
pub struct Instruction {
    opcode: Opcode,
    immediate: u32,
}

impl Instruction {
    pub fn new(opcode: Opcode, immediate: u32) -> Self {
        Self { opcode, immediate }
    }
    pub fn encode_instruction(opcode: &Opcode, immediate: u32) -> u32 {
        *opcode as u32 | immediate!(immediate)
    }
    pub fn decode_instruction(bytecode: u32) -> Self {
        Instruction::new(
            Opcode::match_opcode((bytecode >> 24) as u8),
            sign_extend!(immediate!(bytecode)),
        )
    }
}

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
    if env::args().count() < 2 {
        println!("Ninja Virtual Machine started");
        println!("Ninja Virtual Machine stopped");
        let result: u32 = immediate!(2);
        let extend: u32 = sign_extend!(immediate!(2));
        println!("{result}");
        println!("{extend}");
    }
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
}

pub mod cpu;
pub use cpu::*;
pub mod memory;
pub use memory::*;
use std::env;
use std::io::stdin;
use std::process::exit;

pub const MAXITEMS: u8 = 100;

pub struct NinjaVM {
    pub stack: Stack,
    pub program_memory: ProgramMemory,
}

impl Default for NinjaVM {
    fn default() -> Self {
        Self::new()
    }
}

impl NinjaVM {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            program_memory: ProgramMemory::new(),
        }
    }
    pub fn init(&self) {
        println!("Ninja Virtual Machine started");
    }
    pub fn work(&mut self) {
        for i in 0..=self.program_memory.pc {
            self.execute(self.program_memory.memory[i as usize]);
        }
    }
    fn execute(&mut self, bytecode: Bytecode) {
        let instruction = Instruction::decode_instruction(bytecode);
        match instruction.opcode {
            Opcode::Halt => self.halt(),
            Opcode::Pushc => self.pushc(instruction.immediate),
            Opcode::Add => self.add(),
            Opcode::Sub => self.sub(),
            Opcode::Mul => self.mul(),
            Opcode::Div => self.div(),
            Opcode::Mod => self.modulo(),
            Opcode::Rdint => self.rdint(),
            Opcode::Wrint => self.wrint(),
            Opcode::Rdchr => self.rdchr(),
            Opcode::Wrchr => self.wrchr(),
        }
    }
    fn halt(&self) {
        println!("Ninja Virtual Machine stopped");
        exit(0);
    }
    fn pushc(&mut self, immediate: Immediate) {
        self.stack.push(immediate);
    }
    fn add(&mut self) {
        let n2 = self.stack.pop();
        let n1 = self.stack.pop();
        self.stack.push(n1 + n2);
    }
    fn sub(&mut self) {
        let n2 = self.stack.pop();
        let n1 = self.stack.pop();
        self.stack.push(n1 - n2);
    }
    fn mul(&mut self) {
        let n2 = self.stack.pop();
        let n1 = self.stack.pop();
        self.stack.push(n1 * n2);
    }
    fn div(&mut self) {
        let n2 = self.stack.pop();
        let n1 = self.stack.pop();
        if n2 == 0 {
            panic!("Division by zero error");
        }
        self.stack.push(n1 / n2);
    }
    fn modulo(&mut self) {
        let n2 = self.stack.pop();
        let n1 = self.stack.pop();
        if n2 == 0 {
            panic!("Division by zero error");
        }
        self.stack.push(n1 % n2);
    }
    fn rdint(&mut self) {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let immediate: Immediate = input.trim().parse::<i32>().expect("Input not an integer");
        self.stack.push(immediate)
    }
    fn wrint(&mut self) {
        print!("{}", self.stack.pop())
    }
    fn rdchr(&mut self) {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let immediate = input
            .trim()
            .chars()
            .next()
            .expect("Failed to read character") as Immediate;
        self.stack.push(immediate)
    }
    fn wrchr(&mut self) {
        let character = self.stack.pop() as u8 as char;
        print!("{character}")
    }
}

fn main() {
    let mut vm = NinjaVM::new();
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
            vm.init();
            vm.program_memory.load_prog1();
            vm.work()
        }
        if arg == "--prog2" {
            vm.init();
            vm.program_memory.load_prog2();
            vm.work()
        }
        if arg == "--prog3" {
            vm.init();
            vm.program_memory.load_prog3();
            vm.work()
        }
        println!("unknown command line argument '{arg}', try './njvm --help'");
        exit(1);
    }
}

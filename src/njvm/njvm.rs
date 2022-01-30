use crate::{Bytecode, Immediate, Instruction, Opcode, ProgramMemory, Stack};
use std::io::stdin;
use std::process::exit;

pub const MAXITEMS: u8 = 100;

pub struct NinjaVM {
    pub stack: Stack,
    pub program_memory: ProgramMemory,
}

impl NinjaVM {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            program_memory: ProgramMemory::new(),
        }
    }
    pub fn init() {
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
            println!("Division by zero error");
            exit(1)
        }
        self.stack.push(n1 / n2);
    }
    fn modulo(&mut self) {
        let n2 = self.stack.pop();
        let n1 = self.stack.pop();
        if n2 == 0 {
            println!("Division by zero error");
            exit(1)
        }
        self.stack.push(n1 % n2);
    }
    fn rdint(&mut self) {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let line: i32 = input.trim().parse::<i32>().expect("Input not an integer");
        self.stack.push(line)
    }
    fn wrint(&mut self) {
        print!("{}", self.stack.pop())
    }
    fn rdchr(&mut self) {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let line = input
            .trim()
            .chars()
            .next()
            .expect("Failed to read character") as i32;
        self.stack.push(line)
    }
    fn wrchr(&mut self) {
        let character = self.stack.pop() as u8 as char;
        print!("{character}")
    }
}

use crate::{Instruction, Opcode, ProgramMemory, Stack};
use std::io::stdin;
use std::process::exit;

pub struct Worker;

impl Worker {
    pub fn init() -> (Stack, ProgramMemory) {
        println!("Ninja Virtual Machine started");
        (Stack::new(), ProgramMemory::new())
    }
    pub fn execute(bytecode: u32, stack: &mut Stack) {
        let instruction = Instruction::decode_instruction(bytecode);
        match instruction.opcode {
            Opcode::Halt => Worker::halt(),
            Opcode::Pushc => Worker::pushc(stack, instruction.immediate),
            Opcode::Add => Worker::add(stack),
            Opcode::Sub => Worker::sub(stack),
            Opcode::Mul => Worker::mul(stack),
            Opcode::Div => Worker::div(stack),
            Opcode::Mod => Worker::modulo(stack),
            Opcode::Rdint => Worker::rdint(stack),
            Opcode::Wrint => Worker::wrint(stack),
            Opcode::Rdchr => Worker::rdchr(stack),
            Opcode::Wrchr => Worker::wrchr(stack),
        }
    }
    pub fn work(program_memory: &mut ProgramMemory, stack: &mut Stack) {
        for i in 0..program_memory.pc {
            Worker::execute(program_memory.memory[i as usize], stack);
        }
    }
    fn halt() {
        println!("Ninja Virtual Machine stopped");
        exit(0);
    }
    fn pushc(stack: &mut Stack, immediate: u32) {
        stack.push(immediate);
    }
    fn add(stack: &mut Stack) {
        let n2 = stack.pop();
        let n1 = stack.pop();
        stack.push(n1 + n2);
    }
    fn sub(stack: &mut Stack) {
        let n2 = stack.pop();
        let n1 = stack.pop();
        stack.push(n1 - n2);
    }
    fn mul(stack: &mut Stack) {
        let n2 = stack.pop();
        let n1 = stack.pop();
        stack.push(n1 * n2);
    }
    fn div(stack: &mut Stack) {
        let n2 = stack.pop();
        let n1 = stack.pop();
        if n2 == 0 {
            println!("Division by zero error");
            exit(1)
        }
        stack.push(n1 / n2);
    }
    fn modulo(stack: &mut Stack) {
        let n2 = stack.pop();
        let n1 = stack.pop();
        if n2 == 0 {
            println!("Division by zero error");
            exit(1)
        }
        stack.push(n1 % n2);
    }
    fn rdint(stack: &mut Stack) {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let line: u32 = input.trim().parse::<u32>().expect("Input not an integer");
        stack.push(line)
    }
    fn wrint(stack: &mut Stack) {
        print!("{}", stack.pop())
    }
    fn rdchr(stack: &mut Stack) {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let line = input.trim().chars().next().expect("Failed to read character") as u32;
        stack.push(line)
    }
    fn wrchr(stack: &mut Stack) {
        let character = stack.pop() as u8 as char;
        print!("{character}")
    }
}

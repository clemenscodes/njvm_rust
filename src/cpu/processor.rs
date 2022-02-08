use crate::{Bytecode, Immediate, Instruction, NinjaVM, Opcode, ProgramMemory, Stack, VERSION};
use std::fs::read;
use std::io::{BufRead, Write};
use std::process::exit;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Processor<R, W> {
    pub stack: Stack,
    pub program_memory: ProgramMemory,
    reader: R,
    writer: W,
}

impl<R, W> Processor<R, W>
where
    R: BufRead,
    W: Write,
{
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            stack: Stack::default(),
            program_memory: ProgramMemory::default(),
            reader,
            writer,
        }
    }
    pub fn execute(&mut self, bytecode: Bytecode) {
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
            Opcode::Pushg => self.pushg(instruction.immediate),
            Opcode::Popg => self.popg(instruction.immediate),
            Opcode::Asf => self.asf(instruction.immediate),
            Opcode::Rsf => self.rsf(),
            Opcode::Pushl => self.pushl(instruction.immediate),
            Opcode::Popl => self.popl(instruction.immediate),
        }
    }
    fn halt(&self) {
        println!("Ninja Virtual Machine stopped");
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
        self.reader.read_line(&mut input).expect("Failed to read line");
        let immediate: Immediate = input.trim().parse::<i32>().expect("Input not an integer");
        self.stack.push(immediate)
    }
    fn wrint(&mut self) {
        write!(self.writer, "{}", self.stack.pop()).expect("Unable to write")
    }
    fn rdchr(&mut self) {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed to read line");
        let immediate = input.trim().chars().next().expect("Failed to read character") as Immediate;
        self.stack.push(immediate)
    }
    fn wrchr(&mut self) {
        let character = self.stack.pop() as u8 as char;
        write!(self.writer, "{character}").expect("Unable to write")
    }
    fn pushg(&mut self, immediate: Immediate) {
        println!("Called pushg with immediate {immediate}");
    }
    fn popg(&mut self, immediate: Immediate) {
        println!("Called popg with immediate {immediate}");
    }
    fn asf(&mut self, immediate: Immediate) {
        println!("Called asf with immediate {immediate}");
    }
    fn rsf(&mut self) {
        println!("Called rsf");
    }
    fn pushl(&mut self, immediate: Immediate) {
        println!("Called pushl with immediate {immediate}");
    }
    fn popl(&mut self, immediate: Immediate) {
        println!("Called popl with immediate {immediate}");
    }
    pub fn execute_binary(&mut self, arg: &str) {
        if arg.starts_with('-') {
            NinjaVM::<R, W>::unknown_arg(arg)
        }
        let mut file = match read(arg) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Error: cannot open code file '{arg}'");
                exit(1);
            }
        };
        let mut instructions = file.split_off(16);
        if file.len() < 16 {
            eprintln!("Error: code file is corrupted'");
            exit(1);
        }
        let ninja_binary_format = &[78, 74, 66, 70];
        if !file.starts_with(ninja_binary_format) {
            eprintln!("Error: file '{arg}' is not a Ninja binary");
            exit(1);
        }
        let version = file
            .chunks_mut(4)
            .nth(1)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .expect("Failed to read version");
        if VERSION != version {
            eprintln!("Error: invalid version");
            exit(1)
        }
        instructions.chunks_mut(4).for_each(|c| {
            let instruction = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
            let instruction = Instruction::decode_instruction(instruction);
            let opcode = instruction.opcode;
            let immediate = instruction.immediate;
            self.program_memory.register_instruction(opcode, immediate);
        });
        NinjaVM::<R, W>::init();
        self.program_memory.print();
    }
    pub fn work(&mut self) {
        for i in 0..self.program_memory.pc {
            self.execute(self.program_memory.memory[i as usize]);
        }
        self.program_memory = ProgramMemory::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{stdin, stdout};
    #[test]
    fn test_execute_binary() {
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        cpu.execute_binary("tests/data/a2/prog2.bin");
    }
    #[test]
    fn test_execute() {
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        let instruction = Instruction::encode_instruction(Opcode::Pushc, 1);
        cpu.execute(instruction);
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], 1);
    }
    #[test]
    fn test_pushc() {
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        cpu.pushc(2);
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], 2);
    }
    #[test]
    fn test_add() {
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        cpu.pushc(-1);
        cpu.pushc(2);
        cpu.add();
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], 1);
    }
    #[test]
    fn test_sub() {
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        cpu.pushc(1);
        cpu.pushc(2);
        cpu.sub();
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], -1);
    }
    #[test]
    fn test_mul() {
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        cpu.pushc(-1);
        cpu.pushc(-2);
        cpu.mul();
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], 2);
    }
    #[test]
    fn test_div() {
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        cpu.pushc(-7);
        cpu.pushc(-2);
        cpu.div();
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], 3);
        cpu.pushc(-3);
        cpu.div();
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], -1);
    }
    #[test]
    #[should_panic(expected = "Division by zero error")]
    fn test_division_by_zero_should_fail() {
        std::panic::set_hook(Box::new(|_| {}));
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        cpu.pushc(-2);
        cpu.pushc(4);
        cpu.pushc(-4);
        cpu.add();
        cpu.div();
    }
    #[test]
    fn test_modulo() {
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        cpu.pushc(-9);
        cpu.pushc(4);
        cpu.modulo();
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], -1);
    }
    #[test]
    #[should_panic(expected = "Division by zero error")]
    fn test_modulo_with_zero_should_fail() {
        std::panic::set_hook(Box::new(|_| {}));
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        cpu.pushc(-2);
        cpu.pushc(4);
        cpu.pushc(-4);
        cpu.add();
        cpu.modulo();
    }
    #[test]
    fn test_rdint() {
        let input = b"1";
        let mut cpu = Processor::new(&input[..], stdout());
        cpu.rdint();
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], 1)
    }
    #[test]
    fn test_wrint() {
        let stdin = stdin();
        let mut output = Vec::new();
        let mut cpu = Processor::new(stdin.lock(), &mut output);
        let immediate: Immediate = 42;
        cpu.pushc(immediate);
        cpu.wrint();
        let output = String::from_utf8(output).expect("Not utf-8");
        assert_eq!(output, String::from("42"));
    }
}

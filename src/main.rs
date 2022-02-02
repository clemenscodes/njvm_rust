pub mod cpu;
pub use cpu::*;
pub mod memory;
pub use memory::*;
use std::env;
use std::io::stdin;
use std::process::exit;

pub const MAXITEMS: u8 = 100;

#[derive(Debug, Eq, PartialEq)]
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
        println!("Ninja Virtual Machine started");
        Self {
            stack: Stack::default(),
            program_memory: ProgramMemory::default(),
        }
    }
    pub fn no_arg() {
        eprintln!("Error: no code file specified");
        exit(1)
    }
    pub fn check_arg(arg: &str) {
        match arg {
            "--help" => NinjaVM::help(),
            "--version" => NinjaVM::version(),
            _ => NinjaVM::execute_binary(arg),
        }
    }
    pub fn unknown_arg(arg: &str) {
        eprintln!("unknown command line argument '{arg}', try './njvm --help'");
        exit(1);
    }
    pub fn kill() {
        NinjaVM::help();
        exit(1)
    }
    pub fn help() {
        println!("usage: ./njvm [option] [option] ...");
        println!("  --prog1          select program 1 to execute");
        println!("  --prog2          select program 2 to execute");
        println!("  --prog3          select program 3 to execute");
        println!("  --version        show version and exit");
        println!("  --help           show this help and exit");
    }
    pub fn version() {
        println!("Ninja Virtual Machine version 1 (compiled Sep 23 2015, 10:36:52)");
        exit(0);
    }
    pub fn prog1() {
        let mut vm = NinjaVM::default();
        vm.program_memory.load_prog1();
        vm.work()
    }
    pub fn prog2() {
        let mut vm = NinjaVM::default();
        vm.program_memory.load_prog2();
        vm.work()
    }
    pub fn prog3() {
        let mut vm = NinjaVM::default();
        vm.program_memory.load_prog3();
        vm.work()
    }
    fn execute_binary(arg: &str) {
        if arg.starts_with('-') {
            NinjaVM::unknown_arg(arg)
        }
        println!("Executing binary: {arg}")
    }
    pub fn work(&mut self) {
        for i in 0..self.program_memory.pc {
            self.execute(self.program_memory.memory[i as usize]);
        }
        self.program_memory = ProgramMemory::default();
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
        let immediate = input.trim().chars().next().expect("Failed to read character") as Immediate;
        self.stack.push(immediate)
    }
    fn wrchr(&mut self) {
        let character = self.stack.pop() as u8 as char;
        print!("{character}")
    }
}

fn main() {
    match env::args().len() {
        1 => NinjaVM::no_arg(),
        2 => NinjaVM::check_arg(&env::args().nth(1).expect("Failed to parse argument")),
        _ => NinjaVM::kill(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ninja_vm() {
        let vm = NinjaVM::default();
        assert_eq!(vm.stack.sp, 0);
        assert_eq!(vm.stack.memory.len(), 100);
        assert_eq!(vm.stack.memory[0], 0);
        assert_eq!(vm.stack.memory[99], 0);
        assert_eq!(vm.program_memory.pc, 0);
        assert_eq!(vm.program_memory.memory.len(), 100);
        assert_eq!(vm.program_memory.memory[0], 0);
        assert_eq!(vm.program_memory.memory[99], 0);
    }
    #[test]
    fn test_work() {
        let mut vm = NinjaVM::default();
        vm.program_memory.register_instruction(Opcode::Pushc, 1);
        vm.program_memory.register_instruction(Opcode::Pushc, 2);
        vm.program_memory.register_instruction(Opcode::Add, 0);
        vm.work();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], 3);
        assert_eq!(vm.program_memory, ProgramMemory::default());
    }
    #[test]
    fn test_execute() {
        let mut vm = NinjaVM::default();
        let instruction = Instruction::encode_instruction(Opcode::Pushc, 1);
        vm.execute(instruction);
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], 1);
    }
    #[test]
    fn test_pushc() {
        let mut vm = NinjaVM::default();
        vm.pushc(2);
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], 2);
    }
    #[test]
    fn test_add() {
        let mut vm = NinjaVM::default();
        vm.pushc(-1);
        vm.pushc(2);
        vm.add();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], 1);
    }
    #[test]
    fn test_sub() {
        let mut vm = NinjaVM::default();
        vm.pushc(1);
        vm.pushc(2);
        vm.sub();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], -1);
    }
    #[test]
    fn test_mul() {
        let mut vm = NinjaVM::default();
        vm.pushc(-1);
        vm.pushc(-2);
        vm.mul();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], 2);
    }
    #[test]
    fn test_div() {
        let mut vm = NinjaVM::default();
        vm.pushc(-7);
        vm.pushc(-2);
        vm.div();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], 3);
        vm.pushc(-3);
        vm.div();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], -1);
    }
    #[test]
    #[should_panic(expected = "Division by zero error")]
    fn test_division_by_zero_should_fail() {
        std::panic::set_hook(Box::new(|_| {}));
        let mut vm = NinjaVM::default();
        vm.pushc(-2);
        vm.pushc(4);
        vm.pushc(-4);
        vm.add();
        vm.div();
    }
    #[test]
    fn test_modulo() {
        let mut vm = NinjaVM::default();
        vm.pushc(-9);
        vm.pushc(4);
        vm.modulo();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], -1);
    }
    #[test]
    #[should_panic(expected = "Division by zero error")]
    fn test_modulo_with_zero_should_fail() {
        std::panic::set_hook(Box::new(|_| {}));
        let mut vm = NinjaVM::default();
        vm.pushc(-2);
        vm.pushc(4);
        vm.pushc(-4);
        vm.add();
        vm.modulo();
    }
}

use crate::{Bytecode, Immediate, Instruction, InstructionRegister, Opcode::*, Stack, StaticDataArea};
use std::env::args;
use std::fmt::Debug;
use std::io::{BufRead, Write};
pub mod cpu;
pub use cpu::*;
pub mod memory;
pub use memory::*;
pub mod utils;
pub use utils::*;

pub type Breakpoint = usize;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NinjaVM<R, W> {
    pub stack: Stack<Immediate>,
    pub ir: InstructionRegister,
    pub sda: StaticDataArea<Immediate>,
    pub reader: R,
    pub writer: W,
    pub bp: Option<Breakpoint>,
}

impl Default for NinjaVM<std::io::StdinLock<'_>, std::io::StdoutLock<'_>> {
    fn default() -> Self {
        let stdin = Box::leak(Box::new(std::io::stdin()));
        let stdout = Box::leak(Box::new(std::io::stdout()));
        NinjaVM::new(stdin.lock(), stdout.lock())
    }
}

impl<R: BufRead + Debug, W: Write + Debug> NinjaVM<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            stack: Stack::default(),
            ir: InstructionRegister::default(),
            sda: StaticDataArea::default(),
            reader,
            writer,
            bp: None,
        }
    }
    pub fn execute_instruction(&mut self, bytecode: Bytecode) {
        let instruction = Instruction::decode_instruction(bytecode);
        let immediate = instruction.immediate;
        match instruction.opcode {
            Halt => self.halt(),
            Pushc => self.pushc(immediate),
            Add => self.add(),
            Sub => self.sub(),
            Mul => self.mul(),
            Div => self.div(),
            Mod => self.modulo(),
            Rdint => self.rdint(),
            Wrint => self.wrint(),
            Rdchr => self.rdchr(),
            Wrchr => self.wrchr(),
            Pushg => self.pushg(immediate),
            Popg => self.popg(immediate),
            Asf => self.asf(immediate),
            Rsf => self.rsf(),
            Pushl => self.pushl(immediate),
            Popl => self.popl(immediate),
            Eq => self.eq(),
            Ne => self.ne(),
            Lt => self.lt(),
            Le => self.le(),
            Gt => self.gt(),
            Ge => self.ge(),
            Jmp => self.jmp(immediate),
            Brf => self.brf(immediate),
            Brt => self.brt(immediate),
            Call => self.call(immediate),
            Ret => self.ret(),
            Drop => self.drop(immediate),
        }
    }
    pub fn work(&mut self) {
        loop {
            let instruction = self.ir.data[self.ir.pc];
            let decoded = Instruction::decode_instruction(instruction);
            let opcode = decoded.opcode;
            self.ir.pc += 1;
            self.execute_instruction(instruction);
            if opcode == Halt {
                break;
            }
        }
    }
    pub fn execute_binary(&mut self, bin: &str) {
        let instructions = self.load_binary(bin);
        self.load_instructions(&instructions);
        self.init();
        self.work();
    }
    pub fn load_binary(&mut self, arg: &str) -> Vec<u8> {
        verify_arg(arg);
        let mut file = read_file(arg);
        let instructions = split_file_metadata(&mut file);
        check_ninja_format(&file, arg);
        check_ninja_version(&file);
        let variable_count = check_variables(&file);
        let instruction_count = check_instructions(&file);
        self.sda = StaticDataArea::new(variable_count, 0);
        self.ir = InstructionRegister::new(instruction_count, 0);
        instructions
    }
    pub fn load_test_binary(&mut self, arg: &str) -> Vec<u8> {
        verify_arg(arg);
        let mut file = read_file(arg);
        let instructions = split_file_metadata(&mut file);
        check_ninja_format(&file, arg);
        set_ninja_version(&mut file);
        let variable_count = check_variables(&file);
        let instruction_count = check_instructions(&file);
        self.sda = StaticDataArea::new(variable_count, 0);
        self.ir = InstructionRegister::new(instruction_count, 0);
        instructions
    }
    pub fn load_instructions(&mut self, instructions: &[u8]) {
        instructions.chunks(4).for_each(|c| {
            let instruction = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
            let instruction = Instruction::decode_instruction(instruction);
            let opcode = instruction.opcode;
            let immediate = instruction.immediate;
            self.ir.register_instruction(opcode, immediate);
        });
    }
    pub fn load(&mut self, bin: &str) {
        let instructions = self.load_binary(bin);
        self.load_instructions(&instructions)
    }
    pub fn init(&mut self) {
        println!("Ninja Virtual Machine started");
        self.ir.pc = 0;
    }
}

fn main() {
    let mut vm = NinjaVM::default();
    match args().len() {
        1 => fatal_error("Error: no code file specified"),
        2 => {
            let arg = &args().nth(1).unwrap();
            match arg as &str {
                "--help" => help(),
                "--version" => version(),
                "--debug" => fatal_error("Error: no code file specified"),
                _ => {
                    if arg.starts_with('-') {
                        unknown_arg(arg)
                    }
                    vm.execute_binary(arg)
                }
            }
        }
        3 => {
            let first_arg = &args().nth(1).unwrap();
            let second_arg = &args().nth(2).unwrap();
            match first_arg as &str {
                "--help" => help(),
                "--version" => version(),
                "--debug" => match second_arg as &str {
                    "--help" => help(),
                    "--version" => version(),
                    "--debug" => fatal_error("Error: no code file specified"),
                    _ => {
                        if second_arg.starts_with('-') {
                            unknown_arg(second_arg)
                        }
                        vm.debug(second_arg)
                    }
                },
                _ => match second_arg as &str {
                    "--help" => help(),
                    "--version" => version(),
                    "--debug" => vm.debug(first_arg),
                    _ => {
                        if second_arg.starts_with('-') {
                            unknown_arg(second_arg)
                        }
                        fatal_error("Error: more than one code file specified")
                    }
                },
            }
        }
        _ => kill(),
    }
}

pub fn help() {
    println!("usage: ./njvm [options] <code file>");
    println!("Options:");
    println!("  --debug          start virtual machine in debug mode");
    println!("  --version        show version and exit");
    println!("  --help           show this help and exit");
}

pub fn version() {
    println!(
        "Ninja Virtual Machine version {} (compiled Sep 23 2015, 10:36:52)",
        VERSION
    );
}

fn kill() {
    help();
    #[cfg(not(test))]
    std::process::exit(1);
    #[cfg(test)]
    panic!();
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, InstructionRegister, NinjaVM, Opcode::*};
    #[test]
    fn test_ninja_vm() {
        let vm = NinjaVM::default();
        assert_eq!(vm.stack.sp, 0);
        assert_eq!(vm.stack.data.len(), 0);
        assert_eq!(vm.ir.pc, 0);
        assert_eq!(vm.ir.data.len(), 0);
    }
    #[test]
    fn test_work() {
        let mut vm = NinjaVM::default();
        vm.ir = InstructionRegister::new(3, 0);
        vm.ir.register_instruction(Pushc, 1);
        vm.ir.register_instruction(Pushc, 2);
        vm.ir.register_instruction(Halt, 0);
        vm.init();
        vm.work();
        assert_eq!(vm.stack.data.len(), 2);
    }
    #[test]
    fn test_execute_instruction() {
        let mut vm = NinjaVM::default();
        let instruction = Instruction::encode_instruction(Pushc, 1);
        vm.execute_instruction(instruction);
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.data[0], 1);
    }
    #[test]
    fn test_load_instruction() {
        let mut vm = NinjaVM::default();
        let mut instructions = Vec::new();
        vm.load_instructions(&mut instructions);
    }
    #[test]
    #[should_panic(expected = "Error: cannot open code file 'tests/data/a2/prog1.404'")]
    fn test_load_binary_fails() {
        let mut vm = NinjaVM::default();
        let path = "tests/data/a2/prog1.404";
        vm.load_binary(path);
    }
}

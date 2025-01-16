pub mod cpu;
pub mod memory;
pub mod utils;

use std::fmt::Debug;
use std::io::{BufRead, Write};

use cpu::immediate::Immediate;
use cpu::instruction::Instruction;
use memory::instruction_register::{Bytecode, InstructionRegister};
use memory::stack::Stack;
use memory::static_data_area::StaticDataArea;
use utils::check_instructions::check_instructions;
use utils::check_ninja_format::check_ninja_format;
use utils::check_ninja_version::{check_ninja_version, VERSION};
use utils::check_variables::check_variables;
use utils::fatal_error::fatal_error;
use utils::read_file::read_file;
use utils::set_ninja_version::set_ninja_version;
use utils::split_file_metadata::split_file_metadata;
use utils::unknown_arg::unknown_arg;
use utils::verify_arg::verify_arg;

pub type Breakpoint = usize;
pub type ReturnValueRegister = Immediate;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NinjaVM<R, W> {
    pub stack: Stack<Immediate>,
    pub ir: InstructionRegister,
    pub sda: StaticDataArea<Immediate>,
    pub reader: R,
    pub writer: W,
    pub bp: Option<Breakpoint>,
    pub rv: Option<ReturnValueRegister>,
}

impl Default for NinjaVM<std::io::StdinLock<'_>, std::io::StdoutLock<'_>> {
    fn default() -> Self {
        let stdin = Box::leak(Box::new(std::io::stdin()));
        let stdout = Box::leak(Box::new(std::io::stdout()));
        NinjaVM::new(stdin.lock(), stdout.lock())
    }
}

impl<R: BufRead + Debug, W: Write + Debug> NinjaVM<R, W> {
    pub fn start() {
        let args: Vec<String> = std::env::args().skip(1).collect();

        if args.is_empty() {
            fatal_error("Error: no code file specified");
        }

        let mut debug_mode = false;
        let mut file: Option<String> = None;

        for arg in args {
            match arg.as_str() {
                "--help" => {
                    Self::help();
                    return;
                }
                "--version" => {
                    Self::version();
                    return;
                }
                "--debug" => {
                    if debug_mode {
                        fatal_error("Error: duplicate '--debug' flag");
                    }
                    debug_mode = true;
                }
                _ if arg.starts_with('-') => unknown_arg(&arg),
                _ => {
                    if file.is_some() {
                        fatal_error("Error: more than one code file specified");
                    }
                    file = Some(arg);
                }
            }
        }

        let file = file.unwrap_or_else(|| fatal_error("Error: no code file specified"));

        let mut vm = NinjaVM::default();
        if debug_mode {
            vm.debug(&file);
        } else {
            vm.execute_binary(&file);
        }
    }

    pub fn new(reader: R, writer: W) -> Self {
        Self {
            stack: Stack::default(),
            ir: InstructionRegister::default(),
            sda: StaticDataArea::default(),
            reader,
            writer,
            bp: None,
            rv: None,
        }
    }

    pub fn execute_instruction(&mut self, bytecode: Bytecode) {
        use cpu::opcode::Opcode::*;

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
            Pushr => self.pushr(),
            Popr => self.popr(),
            Dup => self.dup(),
        }
    }

    pub fn work(&mut self) {
        loop {
            let instruction = self.ir.data[self.ir.pc];
            let decoded = Instruction::decode_instruction(instruction);
            let opcode = decoded.opcode;
            self.ir.pc += 1;
            self.execute_instruction(instruction);
            if opcode == cpu::opcode::Opcode::Halt {
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

    fn help() {
        println!("usage: ./njvm [options] <code file>");
        println!("Options:");
        println!("  --debug          start virtual machine in debug mode");
        println!("  --version        show version and exit");
        println!("  --help           show this help and exit");
    }

    fn version() {
        println!("Ninja Virtual Machine version {VERSION} (compiled Sep 23 2015, 10:36:52)",);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpu::opcode::Opcode::*;

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
        let mut vm = NinjaVM {
            ir: InstructionRegister::new(3, 0),
            ..NinjaVM::default()
        };
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
        let instructions = Vec::new();
        vm.load_instructions(&instructions);
    }

    #[test]
    #[should_panic(expected = "Error: cannot open code file 'tests/data/a2/prog1.404'")]
    fn test_load_binary_fails() {
        let mut vm = NinjaVM::default();
        let path = "tests/data/a2/prog1.404";
        vm.load_binary(path);
    }
}

pub mod cpu;
pub use cpu::*;
pub mod memory;
pub use memory::*;
use std::env;
use std::process::exit;

pub type Bytecode = u32;

pub const MAXITEMS: u32 = 10000;

#[derive(Debug, Eq, PartialEq)]
pub struct NinjaVM {
    pub cpu: Processor,
}

impl Default for NinjaVM {
    fn default() -> Self {
        Self::new()
    }
}

impl NinjaVM {
    pub fn new() -> Self {
        Self {
            cpu: Processor::default(),
        }
    }
    pub fn init() {
        println!("Ninja Virtual Machine started");
    }
    pub fn no_arg() {
        eprintln!("Error: no code file specified");
        exit(1)
    }
    pub fn check_arg(&mut self, arg: &str) {
        match arg {
            "--help" => NinjaVM::help(),
            "--version" => NinjaVM::version(),
            _ => self.cpu.execute_binary(arg),
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
        println!("usage: ./njvm [options] <code file>");
        println!("  --version        show version and exit");
        println!("  --help           show this help and exit");
    }
    pub fn version() {
        println!("Ninja Virtual Machine version 2 (compiled Sep 23 2015, 10:36:52)");
        exit(0);
    }
    pub fn work(&mut self) {
        for i in 0..self.cpu.program_memory.pc {
            self.cpu.execute(self.cpu.program_memory.memory[i as usize]);
        }
        self.cpu.program_memory = ProgramMemory::default();
    }
}

fn main() {
    let mut vm = NinjaVM::default();
    match env::args().len() {
        1 => NinjaVM::no_arg(),
        2 => vm.check_arg(&env::args().nth(1).expect("Failed to parse argument")),
        _ => NinjaVM::kill(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ninja_vm() {
        let vm = NinjaVM::default();
        assert_eq!(vm.cpu.stack.sp, 0);
        assert_eq!(vm.cpu.stack.memory.len(), 0);
        assert_eq!(vm.cpu.program_memory.pc, 0);
        assert_eq!(vm.cpu.program_memory.memory.len(), 0);
    }
    #[test]
    fn test_work() {
        let mut vm = NinjaVM::default();
        vm.cpu.program_memory.register_instruction(Opcode::Pushc, 1);
        vm.cpu.program_memory.register_instruction(Opcode::Pushc, 2);
        vm.cpu.program_memory.register_instruction(Opcode::Add, 0);
        vm.work();
        assert_eq!(vm.cpu.stack.sp, 1);
        assert_eq!(vm.cpu.stack.memory[0], 3);
        assert_eq!(vm.cpu.program_memory, ProgramMemory::default());
    }
    #[test]
    fn test_immediate_macro() {
        let instruction_with_opcode: Bytecode = 0xFFFFFFFF;
        let instruction_without_opcode: Bytecode = 0x00000000;
        let opcode_immediate = immediate!(instruction_with_opcode);
        let no_opcode_immediate = immediate!(instruction_without_opcode);
        assert_eq!(opcode_immediate, 0x00FFFFFF);
        assert_eq!(no_opcode_immediate, 0x00000000);
    }
    #[test]
    fn test_sign_extend_macro() {
        let mut positive: Immediate = 0x00000001;
        let mut negative: Immediate = 0x00FFFFFF;
        let positive = sign_extend!(positive);
        let negative = sign_extend!(negative);
        assert_eq!(positive, 1);
        assert_eq!(negative, -1);
    }
}

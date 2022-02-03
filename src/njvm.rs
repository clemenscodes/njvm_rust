use crate::{Processor, VERSION};
use std::process::exit;

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
    pub fn help() {
        println!("usage: ./njvm [options] <code file>");
        println!("  --version        show version and exit");
        println!("  --help           show this help and exit");
    }
    pub fn version() {
        println!("Ninja Virtual Machine version {VERSION} (compiled Sep 23 2015, 10:36:52)");
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
}

#[cfg(test)]
mod tests {
    use crate::{NinjaVM, Opcode, ProgramMemory};
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
        vm.cpu.work();
        assert_eq!(vm.cpu.stack.sp, 1);
        assert_eq!(vm.cpu.stack.memory[0], 3);
        assert_eq!(vm.cpu.program_memory, ProgramMemory::default());
    }
}

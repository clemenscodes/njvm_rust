use crate::{fatal_error, init, unknown_arg, Bytecode, Instruction, Opcode::*, Processor, ProgramMemory, VERSION};
use std::fs;
use std::io::{BufRead, Write};

#[derive(Debug, Eq, PartialEq)]
pub struct NinjaVM<R, W> {
    pub cpu: Processor<R, W>,
}

impl<R, W> NinjaVM<R, W>
where
    R: BufRead,
    W: Write,
{
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            cpu: Processor::new(reader, writer),
        }
    }
    pub fn load_binary(&mut self, arg: &str) {
        self.verify_arg(arg);
        let mut file = self.read_file(arg);
        let mut instructions = self.split_file(&mut file);
        self.check_ninja_format(&mut file, arg);
        self.check_ninja_version(&mut file);
        self.load_instructions(&mut instructions);
    }
    fn verify_arg(&mut self, arg: &str) {
        if arg.starts_with('-') {
            unknown_arg(arg)
        }
    }
    fn read_file(&mut self, arg: &str) -> Vec<u8> {
        match fs::read(arg) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Error: cannot open code file '{arg}'");
                #[cfg(not(test))]
                std::process::exit(1);
                #[cfg(test)]
                panic!("Error: cannot open code file '{arg}'");
            }
        }
    }
    fn split_file(&mut self, file: &mut Vec<u8>) -> Vec<u8> {
        let instructions = file.split_off(16);
        if file.len() < 16 {
            fatal_error("Error: code file is corrupted")
        }
        instructions
    }
    fn check_ninja_format(&mut self, file: &mut Vec<u8>, arg: &str) {
        let ninja_binary_format = &[78, 74, 66, 70];
        if !file.starts_with(ninja_binary_format) {
            eprintln!("Error: file '{arg}' is not a Ninja binary");
            #[cfg(not(test))]
            std::process::exit(1);
            #[cfg(test)]
            panic!("Error: file '{arg}' is not a Ninja binary");
        }
    }
    fn check_ninja_version(&mut self, file: &mut Vec<u8>) {
        let version = match file
            .chunks_mut(4)
            .nth(1)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        {
            Some(version) => version,
            None => fatal_error("Failed to read version"),
        };
        if VERSION != version {
            fatal_error("Error: invalid version")
        }
    }
    fn load_instructions(&mut self, instructions: &mut Vec<u8>) {
        instructions.chunks_mut(4).for_each(|c| {
            let instruction = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
            let instruction = Instruction::decode_instruction(instruction);
            let opcode = instruction.opcode;
            let immediate = instruction.immediate;
            self.cpu.program_memory.register_instruction(opcode, immediate);
        });
    }
    pub fn debug_binary(&mut self, bin: &str) {
        println!("Debugging binary: {bin}");
        self.load_binary(bin);
    }
    pub fn execute_binary(&mut self, bin: &str) {
        self.load_binary(bin);
        init();
        self.cpu.program_memory.print();
        self.work();
    }
    pub fn work(&mut self) {
        for i in 0..self.cpu.program_memory.pc {
            self.execute_instruction(self.cpu.program_memory.memory[i as usize]);
        }
        self.cpu.program_memory = ProgramMemory::default();
    }
    pub fn execute_instruction(&mut self, bytecode: Bytecode) {
        let instruction = Instruction::decode_instruction(bytecode);
        let immediate = instruction.immediate;
        match instruction.opcode {
            Halt => self.cpu.halt(),
            Pushc => self.cpu.pushc(immediate),
            Add => self.cpu.add(),
            Sub => self.cpu.sub(),
            Mul => self.cpu.mul(),
            Div => self.cpu.div(),
            Mod => self.cpu.modulo(),
            Rdint => self.cpu.rdint(),
            Wrint => self.cpu.wrint(),
            Rdchr => self.cpu.rdchr(),
            Wrchr => self.cpu.wrchr(),
            Pushg => self.cpu.pushg(immediate),
            Popg => self.cpu.popg(immediate),
            Asf => self.cpu.asf(immediate),
            Rsf => self.cpu.rsf(),
            Pushl => self.cpu.pushl(immediate),
            Popl => self.cpu.popl(immediate),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, NinjaVM, Opcode, ProgramMemory};
    use std::io::{stdin, stdout};
    #[test]
    fn test_ninja_vm() {
        let stdin = stdin();
        let input = stdin.lock();
        let output = stdout();
        let vm = NinjaVM::new(input, output);
        assert_eq!(vm.cpu.stack.sp, 0);
        assert_eq!(vm.cpu.stack.memory.len(), 0);
        assert_eq!(vm.cpu.program_memory.pc, 0);
        assert_eq!(vm.cpu.program_memory.memory.len(), 0);
    }
    #[test]
    fn test_execute_binary() {
        let stdin = stdin();
        let mut vm = NinjaVM::new(stdin.lock(), stdout());
        vm.load_binary("tests/data/a2/prog2.bin");
    }
    #[test]
    fn test_work() {
        let stdin = stdin();
        let input = stdin.lock();
        let output = stdout();
        let mut vm = NinjaVM::new(input, output);
        vm.cpu.program_memory.register_instruction(Opcode::Pushc, 1);
        vm.cpu.program_memory.register_instruction(Opcode::Pushc, 2);
        vm.cpu.program_memory.register_instruction(Opcode::Add, 0);
        vm.work();
        assert_eq!(vm.cpu.stack.sp, 1);
        assert_eq!(vm.cpu.stack.memory[0], 3);
        assert_eq!(vm.cpu.program_memory, ProgramMemory::default());
    }
    #[test]
    fn test_execute_instruction() {
        let stdin = stdin();
        let mut vm = NinjaVM::new(stdin.lock(), stdout());
        let instruction = Instruction::encode_instruction(Opcode::Pushc, 1);
        vm.execute_instruction(instruction);
        assert_eq!(vm.cpu.stack.sp, 1);
        assert_eq!(vm.cpu.stack.memory[0], 1);
    }
}

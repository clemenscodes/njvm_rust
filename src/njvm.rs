use crate::{init, unknown_arg, Bytecode, Instruction, Opcode::*, Processor, ProgramMemory, VERSION};
use std::fs;
use std::io::{BufRead, Write};
use std::process::exit;

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
    pub fn execute_binary(&mut self, arg: &str) {
        if arg.starts_with('-') {
            unknown_arg(arg)
        }
        let mut file = match fs::read(arg) {
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
            self.cpu.program_memory.register_instruction(opcode, immediate);
        });
        init();
        self.cpu.program_memory.print();
    }
    pub fn debug_binary(&mut self, bin: &str) {
        println!("Debugging binary: {bin}");
    }
    pub fn work(&mut self) {
        for i in 0..self.cpu.program_memory.pc {
            self.execute_instruction(self.cpu.program_memory.memory[i as usize]);
        }
        self.cpu.program_memory = ProgramMemory::default();
    }
    pub fn execute_instruction(&mut self, bytecode: Bytecode) {
        let instruction = Instruction::decode_instruction(bytecode);
        match instruction.opcode {
            Halt => self.cpu.halt(),
            Pushc => self.cpu.pushc(instruction.immediate),
            Add => self.cpu.add(),
            Sub => self.cpu.sub(),
            Mul => self.cpu.mul(),
            Div => self.cpu.div(),
            Mod => self.cpu.modulo(),
            Rdint => self.cpu.rdint(),
            Wrint => self.cpu.wrint(),
            Rdchr => self.cpu.rdchr(),
            Wrchr => self.cpu.wrchr(),
            Pushg => self.cpu.pushg(instruction.immediate),
            Popg => self.cpu.popg(instruction.immediate),
            Asf => self.cpu.asf(instruction.immediate),
            Rsf => self.cpu.rsf(),
            Pushl => self.cpu.pushl(instruction.immediate),
            Popl => self.cpu.popl(instruction.immediate),
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
        vm.execute_binary("tests/data/a2/prog2.bin");
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

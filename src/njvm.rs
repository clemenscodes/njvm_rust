use crate::{utils::*, Bytecode, Instruction, Opcode::*, Processor};
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
    pub fn work(&mut self) {
        for i in 0..self.cpu.instruction_cache.pc {
            self.execute_instruction(self.cpu.instruction_cache.instructions[i as usize]);
        }
    }
    pub fn execute_binary(&mut self, bin: &str) {
        self.load_binary(bin);
        self.init();
        self.cpu.instruction_cache.print();
        self.work();
    }
    pub fn debug_binary(&mut self, bin: &str) {
        println!("Debugging binary: {bin}");
        self.load_binary(bin);
    }
    fn load_binary(&mut self, arg: &str) {
        verify_arg(arg);
        let mut file = read_file(arg);
        let mut instructions = split_file_metadata(&mut file);
        check_ninja_format(&mut file, arg);
        check_ninja_version(&mut file);
        self.load_instructions(&mut instructions);
    }
    fn load_instructions(&mut self, instructions: &mut Vec<u8>) {
        instructions.chunks_mut(4).for_each(|c| {
            let instruction = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
            let instruction = Instruction::decode_instruction(instruction);
            let opcode = instruction.opcode;
            let immediate = instruction.immediate;
            self.cpu.instruction_cache.register_instruction(opcode, immediate);
        });
    }
    fn init(&mut self) {
        println!("Ninja Virtual Machine started");
    }
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, NinjaVM, Opcode::*};
    use std::io::{stdin, stdout};
    #[test]
    fn test_ninja_vm() {
        let stdin = stdin();
        let input = stdin.lock();
        let output = stdout();
        let vm = NinjaVM::new(input, output);
        assert_eq!(vm.cpu.stack.sp, 0);
        assert_eq!(vm.cpu.stack.memory.len(), 0);
        assert_eq!(vm.cpu.instruction_cache.pc, 0);
        assert_eq!(vm.cpu.instruction_cache.instructions.len(), 0);
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
        vm.cpu.instruction_cache.register_instruction(Pushc, 1);
        vm.cpu.instruction_cache.register_instruction(Pushc, 2);
        vm.cpu.instruction_cache.register_instruction(Add, 0);
        vm.work();
        assert_eq!(vm.cpu.stack.sp, 1);
        assert_eq!(vm.cpu.stack.memory[0], 3);
    }
    #[test]
    fn test_execute_instruction() {
        let stdin = stdin();
        let mut vm = NinjaVM::new(stdin.lock(), stdout());
        let instruction = Instruction::encode_instruction(Pushc, 1);
        vm.execute_instruction(instruction);
        assert_eq!(vm.cpu.stack.sp, 1);
        assert_eq!(vm.cpu.stack.memory[0], 1);
    }
    #[test]
    fn test_load_instruction() {
        let stdin = stdin();
        let input = stdin.lock();
        let output = stdout();
        let mut vm = NinjaVM::new(input, output);
        let mut instructions = Vec::new();
        vm.load_instructions(&mut instructions);
    }
    #[test]
    fn test_load_binary() {
        let stdin = stdin();
        let input = stdin.lock();
        let output = stdout();
        let mut vm = NinjaVM::new(input, output);
        let path = "tests/data/a2/prog1.bin";
        vm.load_binary(path);
        assert_eq!(vm.cpu.instruction_cache.instructions.len(), 19)
    }
    #[test]
    #[should_panic(expected = "Error: cannot open code file 'tests/data/a2/prog1.404'")]
    fn test_load_binary_fails() {
        let stdin = stdin();
        let input = stdin.lock();
        let output = stdout();
        let mut vm = NinjaVM::new(input, output);
        let path = "tests/data/a2/prog1.404";
        vm.load_binary(path);
    }
}

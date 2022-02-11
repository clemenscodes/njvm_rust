use crate::{utils::*, Bytecode, Immediate, Instruction, InstructionCache, Opcode::*, Stack, StaticDataArea};
use std::io::{BufRead, Write};

#[derive(Debug, Eq, PartialEq)]
pub struct NinjaVM<R, W> {
    pub stack: Stack<Immediate>,
    pub instruction_cache: InstructionCache<Bytecode>,
    pub sda: StaticDataArea<Immediate>,
    pub reader: R,
    pub writer: W,
}

impl Default for NinjaVM<std::io::StdinLock<'_>, std::io::StdoutLock<'_>> {
    fn default() -> Self {
        let stdin = Box::leak(Box::new(std::io::stdin()));
        let stdout = Box::leak(Box::new(std::io::stdout()));
        NinjaVM::new(stdin.lock(), stdout.lock())
    }
}

impl<R, W> NinjaVM<R, W>
where
    R: BufRead,
    W: Write,
{
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            stack: Stack::default(),
            instruction_cache: InstructionCache::default(),
            sda: StaticDataArea::default(),
            reader,
            writer,
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
        }
    }
    pub fn work(&mut self) {
        self.init();
        for i in 0..self.instruction_cache.pc {
            self.execute_instruction(self.instruction_cache.register[i as usize]);
        }
    }
    pub fn execute_binary(&mut self, bin: &str) {
        let instructions = self.load_binary(bin);
        self.load_instructions(&instructions);
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
        self.instruction_cache = InstructionCache::new(instruction_count, 0);
        instructions
    }
    pub fn load_instructions(&mut self, instructions: &[u8]) {
        instructions.chunks(4).for_each(|c| {
            let instruction = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
            let instruction = Instruction::decode_instruction(instruction);
            let opcode = instruction.opcode;
            let immediate = instruction.immediate;
            self.instruction_cache.register_instruction(opcode, immediate);
        });
    }
    pub fn init(&mut self) {
        println!("Ninja Virtual Machine started");
    }
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, InstructionCache, NinjaVM, Opcode::*};
    #[test]
    fn test_ninja_vm() {
        let vm = NinjaVM::default();
        assert_eq!(vm.stack.sp, 0);
        assert_eq!(vm.stack.memory.len(), 0);
        assert_eq!(vm.instruction_cache.pc, 0);
        assert_eq!(vm.instruction_cache.register.len(), 0);
    }
    #[test]
    fn test_work() {
        let mut vm = NinjaVM::default();
        vm.instruction_cache = InstructionCache::new(3, 0);
        vm.instruction_cache.register_instruction(Pushc, 1);
        vm.instruction_cache.register_instruction(Pushc, 2);
        vm.instruction_cache.register_instruction(Add, 0);
        vm.work();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], 3);
    }
    #[test]
    fn test_execute_instruction() {
        let mut vm = NinjaVM::default();
        let instruction = Instruction::encode_instruction(Pushc, 1);
        vm.execute_instruction(instruction);
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], 1);
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

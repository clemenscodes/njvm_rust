use crate::{Immediate, Instruction, Opcode, Opcode::*};

pub type Bytecode = u32;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct InstructionCache<U> {
    pub pc: usize,
    pub register: Vec<U>,
}

impl Default for InstructionCache<Bytecode> {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl InstructionCache<Bytecode> {
    pub fn new(size: usize, value: Bytecode) -> Self {
        let mut register = vec![];
        register.resize(size, value);
        InstructionCache { pc: 0, register }
    }
    pub fn register_instruction(&mut self, opcode: Opcode, immediate: Immediate) {
        let instruction = Instruction::encode_instruction(opcode, immediate);
        self.register[self.pc] = instruction;
        self.pc += 1;
    }
    pub fn print_instruction(&mut self, pc: usize) {
        let next_instruction = self.register[pc];
        let decoded_instruction = Instruction::decode_instruction(next_instruction);
        let opcode = decoded_instruction.opcode;
        let immediate = decoded_instruction.immediate;
        match opcode {
            Halt => println!("{pc:04}:\thalt"),
            Pushc => println!("{pc:04}:\tpushc\t{immediate}"),
            Add => println!("{pc:04}:\tadd"),
            Sub => println!("{pc:04}:\tsub"),
            Mul => println!("{pc:04}:\tmul"),
            Div => println!("{pc:04}:\tdiv"),
            Mod => println!("{pc:04}:\tmod"),
            Rdint => println!("{pc:04}:\trdint"),
            Wrint => println!("{pc:04}:\twrint"),
            Rdchr => println!("{pc:04}:\trdchr"),
            Wrchr => println!("{pc:04}:\twrchr"),
            Pushg => println!("{pc:04}:\tpushg\t{immediate}"),
            Popg => println!("{pc:04}:\tpopg\t{immediate}"),
            Asf => println!("{pc:04}:\tasf\t{immediate}"),
            Rsf => println!("{pc:04}:\trsf"),
            Pushl => println!("{pc:04}:\tpushl\t{immediate}"),
            Popl => println!("{pc:04}:\tpopl\t{immediate}"),
            Eq => println!("{pc:04}:\teq"),
            Ne => println!("{pc:04}:\tne"),
            Lt => println!("{pc:04}:\tlt"),
            Le => println!("{pc:04}:\tle"),
            Gt => println!("{pc:04}:\tgt"),
            Ge => println!("{pc:04}:\tge"),
            Jmp => println!("{pc:04}:\tjmp\t{immediate}"),
            Brf => println!("{pc:04}:\tbrf\t{immediate}"),
            Brt => println!("{pc:04}:\tbrt\t{immediate}"),
        }
    }
    pub fn print(&mut self) {
        for i in 0..self.register.len() {
            self.print_instruction(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{InstructionCache, Opcode::Pushc};
    #[test]
    fn test_program_memory() {
        let instruction_cache = InstructionCache::default();
        assert_eq!(instruction_cache.pc, 0);
        assert_eq!(instruction_cache.register.len(), 0);
    }
    #[test]
    fn test_register_instruction() {
        let mut instruction_cache = InstructionCache::new(2, 0);
        instruction_cache.register_instruction(Pushc, 1);
        assert_eq!(instruction_cache.pc, 1);
        assert_eq!(instruction_cache.register[0], 0x01000001);
        instruction_cache.register_instruction(Pushc, 2);
        assert_eq!(instruction_cache.pc, 2);
        assert_eq!(instruction_cache.register[1], 0x01000002);
    }
}

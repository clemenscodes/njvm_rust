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
    pub fn print(&self) {
        for i in 0..self.register.len() {
            let instruction: Instruction = Instruction::decode_instruction(self.register[i]);
            let immediate = instruction.immediate;
            match instruction.opcode {
                Halt => println!("{i:04}:\thalt"),
                Pushc => println!("{i:04}:\tpushc\t{immediate}"),
                Add => println!("{i:04}:\tadd"),
                Sub => println!("{i:04}:\tsub"),
                Mul => println!("{i:04}:\tmul"),
                Div => println!("{i:04}:\tdiv"),
                Mod => println!("{i:04}:\tmod"),
                Rdint => println!("{i:04}:\trdint"),
                Wrint => println!("{i:04}:\twrint"),
                Rdchr => println!("{i:04}:\trdchr"),
                Wrchr => println!("{i:04}:\twrchr"),
                Pushg => println!("{i:04}:\tpushg\t{immediate}"),
                Popg => println!("{i:04}:\tpopg\t{immediate}"),
                Asf => println!("{i:04}:\tasf\t{immediate}"),
                Rsf => println!("{i:04}:\trsf"),
                Pushl => println!("{i:04}:\tpushl\t{immediate}"),
                Popl => println!("{i:04}:\tpopl\t{immediate}"),
                Eq => println!("{i:04}:\teq"),
                Ne => println!("{i:04}:\tne"),
                Lt => println!("{i:04}:\tlt"),
                Le => println!("{i:04}:\tle"),
                Gt => println!("{i:04}:\tgt"),
                Ge => println!("{i:04}:\tge"),
                Jmp => println!("{i:04}:\tjmp\t{immediate}"),
                Brf => println!("{i:04}:\tbrf\t{immediate}"),
                Brt => println!("{i:04}:\tbrt\t{immediate}"),
            }
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

use crate::{Immediate, Instruction, Opcode, Opcode::*};

pub type Bytecode = u32;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct InstructionCache<U> {
    pub pc: u32,
    pub register: Vec<U>,
}

impl Default for InstructionCache<Bytecode> {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl InstructionCache<Bytecode> {
    pub fn new(size: usize, value: Bytecode) -> Self {
        let mut register: Vec<Bytecode> = Vec::new();
        register.resize(size, value);
        InstructionCache { pc: 0, register }
    }
    pub fn register_instruction(&mut self, opcode: Opcode, immediate: Immediate) {
        let instruction: Bytecode = Instruction::encode_instruction(opcode, immediate);
        self.register[self.pc as usize] = instruction;
        self.pc += 1;
    }
    pub fn print(&self) {
        for i in 0..self.pc {
            let instruction: Instruction = Instruction::decode_instruction(self.register[i as usize]);
            let immediate = instruction.immediate;
            match instruction.opcode {
                Halt => println!("{i:03}:\thalt"),
                Pushc => println!("{i:03}:\tpushc\t{}", immediate),
                Add => println!("{i:03}:\tadd"),
                Sub => println!("{i:03}:\tsub"),
                Mul => println!("{i:03}:\tmul"),
                Div => println!("{i:03}:\tdiv"),
                Mod => println!("{i:03}:\tmod"),
                Rdint => println!("{i:03}:\trdint"),
                Wrint => println!("{i:03}:\twrint"),
                Rdchr => println!("{i:03}:\trdchr"),
                Wrchr => println!("{i:03}:\twrchr"),
                Pushg => println!("{i:03}:\tpushg\t{}", immediate),
                Popg => println!("{i:03}:\tpopg\t{}", immediate),
                Asf => println!("{i:03}:\tasf\t{}", immediate),
                Rsf => println!("{i:03}:\trsf"),
                Pushl => println!("{i:03}:\tpushl\t{}", immediate),
                Popl => println!("{i:03}:\tpopl\t{}", immediate),
                Eq => println!("{i:03}:\teq"),
                Ne => println!("{i:03}:\tne"),
                Lt => println!("{i:03}:\tlt"),
                Le => println!("{i:03}:\tle"),
                Gt => println!("{i:03}:\tgt"),
                Ge => println!("{i:03}:\tge"),
                Jmp => println!("{i:03}:\tjmp\t{}", immediate),
                Brf => println!("{i:03}:\tbrf\t{}", immediate),
                Brt => println!("{i:03}:\tbrt\t{}", immediate),
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

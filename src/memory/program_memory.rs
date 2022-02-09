use crate::{Bytecode, Immediate, Instruction, Opcode, Opcode::*};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ProgramMemory<U> {
    pub pc: u32,
    pub memory: Vec<U>,
}

impl Default for ProgramMemory<Bytecode>
where
    Bytecode: std::fmt::Debug + std::fmt::Display,
{
    fn default() -> Self {
        Self::new()
    }
}

impl ProgramMemory<Bytecode> {
    pub fn new() -> Self {
        ProgramMemory { pc: 0, memory: vec![] }
    }
    pub fn register_instruction(&mut self, opcode: Opcode, immediate: Immediate) {
        let instruction: Bytecode = Instruction::encode_instruction(opcode, immediate);
        self.memory.push(instruction);
        self.pc += 1;
    }
    pub fn print(&self) {
        for i in 0..self.pc {
            let instruction: Instruction = Instruction::decode_instruction(self.memory[i as usize]);
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
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Opcode::Pushc, ProgramMemory};
    #[test]
    fn test_program_memory() {
        let program_memory = ProgramMemory::default();
        assert_eq!(program_memory.pc, 0);
        assert_eq!(program_memory.memory.len(), 0);
    }
    #[test]
    fn test_register_instruction() {
        let mut program_memory = ProgramMemory::default();
        program_memory.register_instruction(Pushc, 1);
        assert_eq!(program_memory.pc, 1);
        assert_eq!(program_memory.memory[0], 0x01000001);
        program_memory.register_instruction(Pushc, 2);
        assert_eq!(program_memory.pc, 2);
        assert_eq!(program_memory.memory[1], 0x01000002);
    }
}

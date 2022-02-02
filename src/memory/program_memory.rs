use crate::{Bytecode, Immediate, Instruction, Opcode};

#[derive(Debug, Eq, PartialEq)]
pub struct ProgramMemory {
    pub pc: u32,
    pub memory: Vec<u32>,
}

impl Default for ProgramMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgramMemory {
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
            match instruction.opcode {
                Opcode::Halt => println!("{i:03}:\thalt"),
                Opcode::Pushc => println!("{i:03}:\tpushc\t{}", instruction.immediate),
                Opcode::Add => println!("{i:03}:\tadd"),
                Opcode::Sub => println!("{i:03}:\tsub"),
                Opcode::Mul => println!("{i:03}:\tmul"),
                Opcode::Div => println!("{i:03}:\tdiv"),
                Opcode::Mod => println!("{i:03}:\tmod"),
                Opcode::Rdint => println!("{i:03}:\trdint"),
                Opcode::Wrint => println!("{i:03}:\twrint"),
                Opcode::Rdchr => println!("{i:03}:\trdchr"),
                Opcode::Wrchr => println!("{i:03}:\twrchr"),
                Opcode::Pushg => println!("{i:03}:\tpushg\t{}", instruction.immediate),
                Opcode::Popg => println!("{i:03}:\tpopg\t{}", instruction.immediate),
                Opcode::Asf => println!("{i:03}:\tasf\t{}", instruction.immediate),
                Opcode::Rsf => println!("{i:03}:\trsf\t{}", instruction.immediate),
                Opcode::Pushl => println!("{i:03}:\tpushl\t{}", instruction.immediate),
                Opcode::Popl => println!("{i:03}:\tpopl\t{}", instruction.immediate),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Opcode, ProgramMemory};
    #[test]
    fn test_program_memory() {
        let program_memory = ProgramMemory::default();
        assert_eq!(program_memory.pc, 0);
        assert_eq!(program_memory.memory.len(), 0);
    }
    #[test]
    fn test_register_instruction() {
        let mut program_memory = ProgramMemory::default();
        program_memory.register_instruction(Opcode::Pushc, 1);
        assert_eq!(program_memory.pc, 1);
        assert_eq!(program_memory.memory[0], 0x01000001);
        program_memory.register_instruction(Opcode::Pushc, 2);
        assert_eq!(program_memory.pc, 2);
        assert_eq!(program_memory.memory[1], 0x01000002);
    }
}

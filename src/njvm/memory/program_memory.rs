use crate::{Instruction, Opcode, MAXITEMS, Immediate, Bytecode};

#[derive(Debug)]
pub struct ProgramMemory {
    pub pc: u32,
    pub memory: [Bytecode; MAXITEMS as usize],
}
impl ProgramMemory {
    pub fn new() -> Self {
        ProgramMemory {
            pc: 0,
            memory: [0; MAXITEMS as usize],
        }
    }
    pub fn print(&self) {
        for i in 0..=self.pc {
            let instruction: Instruction = Instruction::decode_instruction(self.memory[i as usize]);
            match instruction.opcode {
                Opcode::Halt => println!("{i:03}\thalt"),
                Opcode::Pushc => println!("{i:03}\tpushc\t{}", instruction.immediate),
                Opcode::Add =>  println!("{i:03}\tadd"),
                Opcode::Sub => println!("{i:03}\tsub"),
                Opcode::Mul => println!("{i:03}\tmul"),
                Opcode::Div => println!("{i:03}\tdiv"),
                Opcode::Mod => println!("{i:03}\tmod"),
                Opcode::Rdint => println!("{i:03}\trdint"),
                Opcode::Wrint => println!("{i:03}\twrint"),
                Opcode::Rdchr => println!("{i:03}\trdchr"),
                Opcode::Wrchr => println!("{i:03}\twrint"),
            }
        }
    }
    pub fn register_instruction(&mut self, opcode: Opcode, immediate: Immediate) {
        let instruction: Bytecode = Instruction::encode_instruction(opcode, immediate);
        self.memory[self.pc as usize] = instruction;
        self.pc = self.pc + 1;
    }
}

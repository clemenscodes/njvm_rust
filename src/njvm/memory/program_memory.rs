use crate::{Instruction, MAXITEMS, Opcode};

pub struct ProgramMemory {
    pc: u32,
    memory: [u32; MAXITEMS as usize],
}

impl ProgramMemory {
    pub fn new() -> Self {
        ProgramMemory {
            pc: 0,
            memory: [0; MAXITEMS as usize],
        }
    }
    pub fn print(&self) {
        for i in 0..self.pc {
            let instruction: Instruction = Instruction::decode_instruction(self.memory[i as usize]);
            match instruction.opcode {
                Opcode::Halt => {
                    println!("{i:03}\thalt");
                    break
                },
                Opcode::Pushc => {
                    println!("{i:03}\tpushc\t{}",instruction.immediate);
                    break
                },
                Opcode::Add => {
                    println!("{i:03}\tadd");
                    break
                },
                Opcode::Sub => {
                    println!("{i:03}\tsub");
                    break
                },
                Opcode::Mul => {
                    println!("{i:03}\tmul");
                    break
                },
                Opcode::Div => {
                    println!("{i:03}\tdiv");
                    break
                },
                Opcode::Mod => {
                    println!("{i:03}\tmod");
                    break
                },
                Opcode::Rdint => {
                    println!("{i:03}\trdint");
                    break
                },
                Opcode::Wrint => {
                    println!("{i:03}\twrint");
                    break
                },
                Opcode::Rdchr => {
                    println!("{i:03}\trdchr");
                    break
                },
                Opcode::Wrchr => {
                    println!("{i:03}\twrint");
                    break
                }
            }
            
        }
    }
}

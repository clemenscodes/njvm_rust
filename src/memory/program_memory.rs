use crate::{Bytecode, Immediate, Instruction, Opcode, MAXITEMS};

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
    pub fn register_instruction(&mut self, opcode: Opcode, immediate: Immediate) {
        let instruction: Bytecode = Instruction::encode_instruction(opcode, immediate);
        self.memory[self.pc as usize] = instruction;
        self.pc = self.pc + 1;
    }
    pub fn load_prog1(&mut self) {
        self.register_instruction(Opcode::Pushc, 3);
        self.register_instruction(Opcode::Pushc, 4);
        self.register_instruction(Opcode::Add, 0);
        self.register_instruction(Opcode::Pushc, 10);
        self.register_instruction(Opcode::Pushc, 6);
        self.register_instruction(Opcode::Sub, 0);
        self.register_instruction(Opcode::Mul, 0);
        self.register_instruction(Opcode::Wrint, 0);
        self.register_instruction(Opcode::Pushc, 10);
        self.register_instruction(Opcode::Wrchr, 0);
        self.register_instruction(Opcode::Halt, 0);
        self.print();
    }
    pub fn load_prog2(&mut self) {
        self.register_instruction(Opcode::Pushc, -2);
        self.register_instruction(Opcode::Rdint, 0);
        self.register_instruction(Opcode::Mul, 0);
        self.register_instruction(Opcode::Pushc, 3);
        self.register_instruction(Opcode::Add, 0);
        self.register_instruction(Opcode::Wrint, 0);
        self.register_instruction(Opcode::Pushc, '\n' as i32);
        self.register_instruction(Opcode::Wrchr, 0);
        self.register_instruction(Opcode::Halt, 0);
        self.print();
    }
    pub fn load_prog3(&mut self) {
        self.register_instruction(Opcode::Rdchr, 0);
        self.register_instruction(Opcode::Wrint, 0);
        self.register_instruction(Opcode::Pushc, '\n' as i32);
        self.register_instruction(Opcode::Wrchr, 0);
        self.register_instruction(Opcode::Halt, 0);
        self.print();
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
                Opcode::Wrchr => println!("{i:03}:\twrint"),
            }
        }
    }
}

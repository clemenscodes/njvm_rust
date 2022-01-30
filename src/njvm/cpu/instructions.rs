#[macro_export]
macro_rules! immediate {
    ($e:expr) => {
        ($e) & 0x00FFFFFF
    };
}

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Halt = 0,
    Pushc = 1,
    Add = 2,
    Sub = 3,
    Mul = 4,
    Div = 5,
    Mod = 6,
    Rdint = 7,
    Wrint = 8,
    Rdchr = 9,
    Wrchr = 10,
}

pub type Bytecode = u32;
pub type Immediate = i32;

#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub immediate: Immediate,
}

impl Instruction {
    pub fn new(opcode: Opcode, immediate: Immediate) -> Self {
        Self { opcode, immediate }
    }
    pub fn encode_opcode(opcode: Opcode) -> u32 {
        let encoded_opcode = (opcode as u32) << 24;
        encoded_opcode
    }
    pub fn encode_immediate(immediate: i32) -> u32 {
        const MIN: i32 = -8388608;
        const MAX: i32 = 8388607;
        match immediate {
            MIN..=MAX => {
                let immediate = immediate!(immediate);
                let bytes = immediate.to_be_bytes();
                let immediate = u32::from_be_bytes(bytes);
                immediate
            }
            _ => panic!("Immediate value out of range"),
        }
    }
    pub fn encode_instruction(opcode: Opcode, immediate: i32) -> Bytecode {
        let encoded_opcode = Instruction::encode_opcode(opcode);
        let encoded_immediate = Instruction::encode_immediate(immediate);
        let instruction = encoded_opcode | encoded_immediate;
        instruction
    }
    pub fn decode_opcode(instruction: Bytecode) -> Opcode {
        let opcode = instruction >> 24;
        match opcode {
            0 => Opcode::Halt,
            1 => Opcode::Pushc,
            2 => Opcode::Add,
            3 => Opcode::Sub,
            4 => Opcode::Mul,
            5 => Opcode::Div,
            6 => Opcode::Mod,
            7 => Opcode::Rdint,
            8 => Opcode::Wrint,
            9 => Opcode::Rdchr,
            10 => Opcode::Wrchr,
            _ => panic!("Unknown opcode"),
        }
    }
    pub fn decode_immediate(instruction: Bytecode) -> Immediate {
        let mut immediate: Immediate = (instruction & 0x00FFFFFF) as Immediate;
        if (immediate & 0x00800000) != 0 {
            let mut bytes = immediate.to_be_bytes();
            bytes[0] = 0xFF;
            immediate = i32::from_be_bytes(bytes);
            immediate
        } else {
            immediate
        }
    }
    pub fn decode_instruction(instruction: Bytecode) -> Self {
        let instruction = Instruction::new(
            Instruction::decode_opcode(instruction),
            Instruction::decode_immediate(instruction),
        );
        // instruction.print();
        instruction
    }
    pub fn print(&self) {
        println!("{self:#?}")
    }
}

#[cfg(test)]
mod tests {
    use crate::{Bytecode, Instruction, Opcode};
    #[test]
    fn test_display_binary() {
        testing()
    }
    fn testing() {
        let instruction: Bytecode = Instruction::encode_instruction(Opcode::Pushc, 123424);
        Instruction::decode_immediate(instruction);
    }
}

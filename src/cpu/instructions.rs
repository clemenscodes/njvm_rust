pub type Bytecode = u32;
pub type Immediate = i32;

#[macro_export]
macro_rules! immediate {
    ($e:expr) => {
        ($e) & 0x00FFFFFF
    };
}

#[macro_export]
macro_rules! sign_extend {
    ($e:expr) => {
        if ($e & 0x00800000) != 0 {
            let mut bytes = $e.to_be_bytes();
            bytes[0] = 0xFF;
            $e = i32::from_be_bytes(bytes);
            $e
        } else {
            $e
        }
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

#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub immediate: Immediate,
}

impl Instruction {
    pub fn new(opcode: Opcode, immediate: Immediate) -> Self {
        Self { opcode, immediate }
    }
    pub fn encode_instruction(opcode: Opcode, immediate: Immediate) -> Bytecode {
        Instruction::encode_opcode(opcode) | Instruction::encode_immediate(immediate)
    }
    pub fn decode_instruction(instruction: Bytecode) -> Self {
        Instruction::new(
            Instruction::decode_opcode(instruction),
            Instruction::decode_immediate(instruction),
        )
    }
    pub fn encode_opcode(opcode: Opcode) -> Bytecode {
        (opcode as u32) << 24
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
    pub fn encode_immediate(immediate: i32) -> Bytecode {
        const MIN: i32 = -8388608;
        const MAX: i32 = 8388607;
        match immediate {
            MIN..=MAX => {
                let immediate = immediate!(immediate);
                let bytes = immediate.to_be_bytes();
                u32::from_be_bytes(bytes)
            }
            _ => panic!("Immediate value out of range"),
        }
    }
    pub fn decode_immediate(instruction: Bytecode) -> Immediate {
        let mut immediate: Immediate = (instruction & 0x00FFFFFF) as Immediate;
        sign_extend!(immediate)
    }
    pub fn print(&self) {
        println!("{self:#?}")
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[ignore]
    #[test]
    fn test_immediate_macro() {
        unimplemented!()
    }
    #[ignore]
    #[test]
    fn test_sign_extend_macro() {
        unimplemented!()
    }
    #[ignore]
    #[test]
    fn test_encode_instruction() {
        unimplemented!()
    }
    #[ignore]
    #[test]
    fn test_decode_instruction() {
        unimplemented!()
    }
    #[ignore]
    #[test]
    fn test_encode_opcode() {
        unimplemented!()
    }
    #[ignore]
    #[test]
    fn test_decode_opcode() {
        unimplemented!()
    }
    #[ignore]
    #[test]
    fn test_encode_immediate() {
        unimplemented!()
    }
    #[ignore]
    #[test]
    fn test_decode_immediate() {
        unimplemented!()
    }
}

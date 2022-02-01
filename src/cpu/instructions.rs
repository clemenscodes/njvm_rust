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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

#[derive(Debug, Eq, PartialEq)]
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
    pub fn encode_immediate(immediate: Immediate) -> Bytecode {
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
        let mut immediate: Immediate = (immediate!(instruction)) as Immediate;
        sign_extend!(immediate)
    }
    pub fn print(&self) {
        println!("{self:#?}")
    }
}

#[cfg(test)]
mod tests {
    use crate::{immediate, sign_extend, Bytecode, Immediate, Instruction, Opcode};
    #[test]
    fn test_instruction() {
        let instruction = Instruction::new(Opcode::Pushc, 1);
        assert_eq!(instruction.opcode, Opcode::Pushc);
        assert_eq!(instruction.immediate, 1);
    }
    #[test]
    fn test_immediate_macro() {
        let instruction_with_opcode: Bytecode = 0xFFFFFFFF;
        let instruction_without_opcode: Bytecode = 0x00000000;
        let opcode_immediate = immediate!(instruction_with_opcode);
        let no_opcode_immediate = immediate!(instruction_without_opcode);
        assert_eq!(opcode_immediate, 0x00FFFFFF);
        assert_eq!(no_opcode_immediate, 0x00000000);
    }
    #[test]
    fn test_sign_extend_macro() {
        let mut positive: Immediate = 0x00000001;
        let mut negative: Immediate = 0x00FFFFFF;
        let positive = sign_extend!(positive);
        let negative = sign_extend!(negative);
        assert_eq!(positive, 1);
        assert_eq!(negative, -1);
    }
    #[test]
    fn test_encode_instruction() {
        assert_eq!(Instruction::encode_instruction(Opcode::Pushc, 1), 0x01000001);
        assert_eq!(Instruction::encode_instruction(Opcode::Pushc, -1), 0x01ffffff);
    }
    #[test]
    fn test_decode_instruction() {
        let decoded_instruction = Instruction::decode_instruction(0x01000001);
        assert_eq!(decoded_instruction.opcode, Opcode::Pushc);
        assert_eq!(decoded_instruction.immediate, 1);
        let decoded_instruction = Instruction::decode_instruction(0x01ffffff);
        assert_eq!(decoded_instruction.opcode, Opcode::Pushc);
        assert_eq!(decoded_instruction.immediate, -1);
    }
    #[test]
    fn test_encode_opcode() {
        assert_eq!(Instruction::encode_opcode(Opcode::Halt), 0x00000000);
        assert_eq!(Instruction::encode_opcode(Opcode::Pushc), 0x01000000);
        assert_eq!(Instruction::encode_opcode(Opcode::Add), 0x02000000);
        assert_eq!(Instruction::encode_opcode(Opcode::Sub), 0x03000000);
        assert_eq!(Instruction::encode_opcode(Opcode::Mul), 0x04000000);
        assert_eq!(Instruction::encode_opcode(Opcode::Div), 0x05000000);
        assert_eq!(Instruction::encode_opcode(Opcode::Mod), 0x06000000);
        assert_eq!(Instruction::encode_opcode(Opcode::Rdint), 0x07000000);
        assert_eq!(Instruction::encode_opcode(Opcode::Wrint), 0x08000000);
        assert_eq!(Instruction::encode_opcode(Opcode::Rdchr), 0x09000000);
        assert_eq!(Instruction::encode_opcode(Opcode::Wrchr), 0x0a000000);
    }
    #[test]
    fn test_decode_opcode() {
        assert_eq!(Instruction::decode_opcode(0x0000f001), Opcode::Halt);
        assert_eq!(Instruction::decode_opcode(0x01000f01), Opcode::Pushc);
        assert_eq!(Instruction::decode_opcode(0x02000001), Opcode::Add);
        assert_eq!(Instruction::decode_opcode(0x030000f1), Opcode::Sub);
        assert_eq!(Instruction::decode_opcode(0x04000001), Opcode::Mul);
        assert_eq!(Instruction::decode_opcode(0x0500f001), Opcode::Div);
        assert_eq!(Instruction::decode_opcode(0x06000001), Opcode::Mod);
        assert_eq!(Instruction::decode_opcode(0x07000001), Opcode::Rdint);
        assert_eq!(Instruction::decode_opcode(0x0800f001), Opcode::Wrint);
        assert_eq!(Instruction::decode_opcode(0x0900c0f1), Opcode::Rdchr);
        assert_eq!(Instruction::decode_opcode(0x0a000f01), Opcode::Wrchr);
    }
    #[test]
    #[should_panic(expected = "Unknown opcode")]
    fn test_unknown_opcode() {
        std::panic::set_hook(Box::new(|_| {}));
        Instruction::decode_opcode(0xFF000001);
    }
    #[test]
    fn test_decode_immediate() {
        assert_eq!(Instruction::decode_immediate(0x00000001), 1);
        assert_eq!(Instruction::decode_immediate(0x00ffffff), -1)
    }
    #[test]
    fn test_encode_immediate() {
        assert_eq!(Instruction::encode_immediate(1), 0x00000001);
        assert_eq!(Instruction::encode_immediate(-1), 0x00ffffff)
    }
    #[test]
    #[should_panic(expected = "Immediate value out of range")]
    fn test_immediate_value_over_range() {
        std::panic::set_hook(Box::new(|_| {}));
        Instruction::encode_immediate(100000000);
    }
    #[test]
    #[should_panic(expected = "Immediate value out of range")]
    fn test_immediate_value_under_range() {
        std::panic::set_hook(Box::new(|_| {}));
        Instruction::encode_immediate(-100000000);
    }
}

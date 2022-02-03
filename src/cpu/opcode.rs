use crate::Bytecode;

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
    Pushg = 11,
    Popg = 12,
    Asf = 13,
    Rsf = 14,
    Pushl = 15,
    Popl = 16,
}

impl Opcode {
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
            11 => Opcode::Pushg,
            12 => Opcode::Popg,
            13 => Opcode::Asf,
            14 => Opcode::Rsf,
            15 => Opcode::Pushl,
            16 => Opcode::Popl,
            _ => panic!("Unknown opcode"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode_opcode() {
        assert_eq!(Opcode::encode_opcode(Opcode::Halt), 0x00000000);
        assert_eq!(Opcode::encode_opcode(Opcode::Pushc), 0x01000000);
        assert_eq!(Opcode::encode_opcode(Opcode::Add), 0x02000000);
        assert_eq!(Opcode::encode_opcode(Opcode::Sub), 0x03000000);
        assert_eq!(Opcode::encode_opcode(Opcode::Mul), 0x04000000);
        assert_eq!(Opcode::encode_opcode(Opcode::Div), 0x05000000);
        assert_eq!(Opcode::encode_opcode(Opcode::Mod), 0x06000000);
        assert_eq!(Opcode::encode_opcode(Opcode::Rdint), 0x07000000);
        assert_eq!(Opcode::encode_opcode(Opcode::Wrint), 0x08000000);
        assert_eq!(Opcode::encode_opcode(Opcode::Rdchr), 0x09000000);
        assert_eq!(Opcode::encode_opcode(Opcode::Wrchr), 0x0a000000);
        assert_eq!(Opcode::encode_opcode(Opcode::Pushg), 0x0b000000);
        assert_eq!(Opcode::encode_opcode(Opcode::Popg), 0x0c000000);
        assert_eq!(Opcode::encode_opcode(Opcode::Asf), 0x0d000000);
        assert_eq!(Opcode::encode_opcode(Opcode::Rsf), 0x0e000000);
        assert_eq!(Opcode::encode_opcode(Opcode::Pushl), 0x0f000000);
        assert_eq!(Opcode::encode_opcode(Opcode::Popl), 0x10000000);
    }
    #[test]
    fn test_decode_opcode() {
        assert_eq!(Opcode::decode_opcode(0x0000f001), Opcode::Halt);
        assert_eq!(Opcode::decode_opcode(0x01000f01), Opcode::Pushc);
        assert_eq!(Opcode::decode_opcode(0x02000001), Opcode::Add);
        assert_eq!(Opcode::decode_opcode(0x030000f1), Opcode::Sub);
        assert_eq!(Opcode::decode_opcode(0x04000001), Opcode::Mul);
        assert_eq!(Opcode::decode_opcode(0x0500f001), Opcode::Div);
        assert_eq!(Opcode::decode_opcode(0x06000001), Opcode::Mod);
        assert_eq!(Opcode::decode_opcode(0x07000001), Opcode::Rdint);
        assert_eq!(Opcode::decode_opcode(0x0800f001), Opcode::Wrint);
        assert_eq!(Opcode::decode_opcode(0x0900c0f1), Opcode::Rdchr);
        assert_eq!(Opcode::decode_opcode(0x0a000f01), Opcode::Wrchr);
        assert_eq!(Opcode::decode_opcode(0x0b000f01), Opcode::Pushg);
        assert_eq!(Opcode::decode_opcode(0x0c000f01), Opcode::Popg);
        assert_eq!(Opcode::decode_opcode(0x0d000f01), Opcode::Asf);
        assert_eq!(Opcode::decode_opcode(0x0e000f01), Opcode::Rsf);
        assert_eq!(Opcode::decode_opcode(0x0f000f01), Opcode::Pushl);
        assert_eq!(Opcode::decode_opcode(0x10000f01), Opcode::Popl);
    }
    #[test]
    #[should_panic(expected = "Unknown opcode")]
    fn test_unknown_opcode() {
        std::panic::set_hook(Box::new(|_| {}));
        Opcode::decode_opcode(0xFF000001);
    }
}

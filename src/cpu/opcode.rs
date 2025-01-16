use crate::{
    memory::instruction_register::Bytecode, utils::fatal_error::fatal_error,
};

#[repr(u8)]
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
    Eq = 17,
    Ne = 18,
    Lt = 19,
    Le = 20,
    Gt = 21,
    Ge = 22,
    Jmp = 23,
    Brf = 24,
    Brt = 25,
    Call = 26,
    Ret = 27,
    Drop = 28,
    Pushr = 29,
    Popr = 30,
    Dup = 31,
}

impl Opcode {
    pub fn encode_opcode(opcode: Opcode) -> Bytecode {
        (opcode as Bytecode) << 24
    }

    pub fn decode_opcode(instruction: Bytecode) -> Self {
        use Opcode::*;
        let opcode = instruction >> 24;
        match opcode {
            0 => Halt,
            1 => Pushc,
            2 => Add,
            3 => Sub,
            4 => Mul,
            5 => Div,
            6 => Mod,
            7 => Rdint,
            8 => Wrint,
            9 => Rdchr,
            10 => Wrchr,
            11 => Pushg,
            12 => Popg,
            13 => Asf,
            14 => Rsf,
            15 => Pushl,
            16 => Popl,
            17 => Eq,
            18 => Ne,
            19 => Lt,
            20 => Le,
            21 => Gt,
            22 => Ge,
            23 => Jmp,
            24 => Brf,
            25 => Brt,
            26 => Call,
            27 => Ret,
            28 => Drop,
            29 => Pushr,
            30 => Popr,
            31 => Dup,
            _ => fatal_error("Unknown opcode"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Opcode::*;

    #[test]
    fn test_encode_opcode() {
        assert_eq!(Opcode::encode_opcode(Halt), 0x00000000);
        assert_eq!(Opcode::encode_opcode(Pushc), 0x01000000);
        assert_eq!(Opcode::encode_opcode(Add), 0x02000000);
        assert_eq!(Opcode::encode_opcode(Sub), 0x03000000);
        assert_eq!(Opcode::encode_opcode(Mul), 0x04000000);
        assert_eq!(Opcode::encode_opcode(Div), 0x05000000);
        assert_eq!(Opcode::encode_opcode(Mod), 0x06000000);
        assert_eq!(Opcode::encode_opcode(Rdint), 0x07000000);
        assert_eq!(Opcode::encode_opcode(Wrint), 0x08000000);
        assert_eq!(Opcode::encode_opcode(Rdchr), 0x09000000);
        assert_eq!(Opcode::encode_opcode(Wrchr), 0x0a000000);
        assert_eq!(Opcode::encode_opcode(Pushg), 0x0b000000);
        assert_eq!(Opcode::encode_opcode(Popg), 0x0c000000);
        assert_eq!(Opcode::encode_opcode(Asf), 0x0d000000);
        assert_eq!(Opcode::encode_opcode(Rsf), 0x0e000000);
        assert_eq!(Opcode::encode_opcode(Pushl), 0x0f000000);
        assert_eq!(Opcode::encode_opcode(Popl), 0x10000000);
        assert_eq!(Opcode::encode_opcode(Eq), 0x11000000);
        assert_eq!(Opcode::encode_opcode(Ne), 0x12000000);
        assert_eq!(Opcode::encode_opcode(Lt), 0x13000000);
        assert_eq!(Opcode::encode_opcode(Le), 0x14000000);
        assert_eq!(Opcode::encode_opcode(Gt), 0x15000000);
        assert_eq!(Opcode::encode_opcode(Ge), 0x16000000);
        assert_eq!(Opcode::encode_opcode(Jmp), 0x17000000);
        assert_eq!(Opcode::encode_opcode(Brf), 0x18000000);
        assert_eq!(Opcode::encode_opcode(Brt), 0x19000000);
        assert_eq!(Opcode::encode_opcode(Call), 0x1a000000);
        assert_eq!(Opcode::encode_opcode(Ret), 0x1b000000);
        assert_eq!(Opcode::encode_opcode(Drop), 0x1c000000);
        assert_eq!(Opcode::encode_opcode(Pushr), 0x1d000000);
        assert_eq!(Opcode::encode_opcode(Popr), 0x1e000000);
        assert_eq!(Opcode::encode_opcode(Dup), 0x1f000000);
    }

    #[test]
    fn test_decode_opcode() {
        assert_eq!(Opcode::decode_opcode(0x0000f001), Halt);
        assert_eq!(Opcode::decode_opcode(0x01000f01), Pushc);
        assert_eq!(Opcode::decode_opcode(0x02000001), Add);
        assert_eq!(Opcode::decode_opcode(0x030000f1), Sub);
        assert_eq!(Opcode::decode_opcode(0x04000001), Mul);
        assert_eq!(Opcode::decode_opcode(0x0500f001), Div);
        assert_eq!(Opcode::decode_opcode(0x06000001), Mod);
        assert_eq!(Opcode::decode_opcode(0x07000001), Rdint);
        assert_eq!(Opcode::decode_opcode(0x0800f001), Wrint);
        assert_eq!(Opcode::decode_opcode(0x0900c0f1), Rdchr);
        assert_eq!(Opcode::decode_opcode(0x0a000f01), Wrchr);
        assert_eq!(Opcode::decode_opcode(0x0b000f01), Pushg);
        assert_eq!(Opcode::decode_opcode(0x0c000f01), Popg);
        assert_eq!(Opcode::decode_opcode(0x0d000f01), Asf);
        assert_eq!(Opcode::decode_opcode(0x0e000f01), Rsf);
        assert_eq!(Opcode::decode_opcode(0x0f000f01), Pushl);
        assert_eq!(Opcode::decode_opcode(0x10000f01), Popl);
        assert_eq!(Opcode::decode_opcode(0x11000000), Eq);
        assert_eq!(Opcode::decode_opcode(0x12000000), Ne);
        assert_eq!(Opcode::decode_opcode(0x13000000), Lt);
        assert_eq!(Opcode::decode_opcode(0x14000000), Le);
        assert_eq!(Opcode::decode_opcode(0x15000000), Gt);
        assert_eq!(Opcode::decode_opcode(0x16000000), Ge);
        assert_eq!(Opcode::decode_opcode(0x17000000), Jmp);
        assert_eq!(Opcode::decode_opcode(0x18000000), Brf);
        assert_eq!(Opcode::decode_opcode(0x19000000), Brt);
        assert_eq!(Opcode::decode_opcode(0x1a000000), Call);
        assert_eq!(Opcode::decode_opcode(0x1b000000), Ret);
        assert_eq!(Opcode::decode_opcode(0x1c000000), Drop);
        assert_eq!(Opcode::decode_opcode(0x1d000000), Pushr);
        assert_eq!(Opcode::decode_opcode(0x1e000000), Popr);
        assert_eq!(Opcode::decode_opcode(0x1f000000), Dup);
    }

    #[test]
    #[should_panic(expected = "Unknown opcode")]
    fn test_unknown_opcode() {
        std::panic::set_hook(Box::new(|_| {}));
        Opcode::decode_opcode(0xFF000001);
    }
}

use crate::Bytecode;

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

pub type Immediate = i32;

pub trait Decoding {
    fn decode_immediate(instruction: Bytecode) -> Immediate;
}

pub trait Encoding {
    fn encode_immediate(immediate: Immediate) -> Bytecode;
}

impl Decoding for Immediate {
    fn decode_immediate(instruction: Bytecode) -> Self {
        let mut immediate: Immediate = (immediate!(instruction)) as Immediate;
        sign_extend!(immediate)
    }
}

impl Encoding for Immediate {
    fn encode_immediate(immediate: Immediate) -> Bytecode {
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
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_decode_immediate() {
        assert_eq!(Immediate::decode_immediate(0x00000001), 1);
        assert_eq!(Immediate::decode_immediate(0x00ffffff), -1)
    }
    #[test]
    fn test_encode_immediate() {
        assert_eq!(Immediate::encode_immediate(1), 0x00000001);
        assert_eq!(Immediate::encode_immediate(-1), 0x00ffffff)
    }
    #[test]
    #[should_panic(expected = "Immediate value out of range")]
    fn test_immediate_value_over_range() {
        std::panic::set_hook(Box::new(|_| {}));
        Immediate::encode_immediate(100000000);
    }
    #[test]
    #[should_panic(expected = "Immediate value out of range")]
    fn test_immediate_value_under_range() {
        std::panic::set_hook(Box::new(|_| {}));
        Immediate::encode_immediate(-100000000);
    }
}

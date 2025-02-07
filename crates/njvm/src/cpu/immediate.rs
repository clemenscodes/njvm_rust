use crate::memory::instruction_register::Bytecode;

#[macro_export]
macro_rules! immediate {
    ($e:expr) => {
        (($e) & 0x00FFFFFF) as Immediate
    };
}

#[macro_export]
macro_rules! sign_extend {
    ($e:expr) => {
        if ($e & 0x00800000) != 0 {
            let mut bytes = $e.to_be_bytes();
            bytes[0] = 0xFF;
            i32::from_be_bytes(bytes)
        } else {
            $e
        }
    };
}

pub type Immediate = i32;

pub trait Decoding {
    fn decode(instruction: Bytecode) -> Immediate;
}

pub trait Encoding {
    fn encode(immediate: Immediate) -> Bytecode;
}

impl Decoding for Immediate {
    fn decode(instruction: Bytecode) -> Self {
        sign_extend!(immediate!(instruction))
    }
}

impl Encoding for Immediate {
    fn encode(immediate: Immediate) -> Bytecode {
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
        let positive: Immediate = 0x00000001;
        let negative: Immediate = 0x00FFFFFF;
        let positive = sign_extend!(positive);
        let negative = sign_extend!(negative);
        assert_eq!(positive, 1);
        assert_eq!(negative, -1);
    }

    #[test]
    fn test_decode_immediate() {
        assert_eq!(Immediate::decode(0x00000001), 1);
        assert_eq!(Immediate::decode(0x00ffffff), -1)
    }

    #[test]
    fn test_encode_immediate() {
        assert_eq!(Immediate::encode(1), 0x00000001);
        assert_eq!(Immediate::encode(-1), 0x00ffffff)
    }

    #[test]
    #[should_panic(expected = "Immediate value out of range")]
    fn test_immediate_value_over_range() {
        std::panic::set_hook(Box::new(|_| {}));
        Immediate::encode(100000000);
    }

    #[test]
    #[should_panic(expected = "Immediate value out of range")]
    fn test_immediate_value_under_range() {
        std::panic::set_hook(Box::new(|_| {}));
        Immediate::encode(-100000000);
    }
}

use crate::cpu::immediate::{Decoding, Encoding, Immediate};
use crate::cpu::opcode::Opcode;
use crate::memory::instruction_register::Bytecode;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Instruction {
    pub opcode: Opcode,
    pub immediate: Immediate,
}

impl Instruction {
    pub fn new(opcode: Opcode, immediate: Immediate) -> Self {
        Self { opcode, immediate }
    }
    pub fn encode_instruction(
        opcode: Opcode,
        immediate: Immediate,
    ) -> Bytecode {
        Opcode::encode_opcode(opcode) | Immediate::encode_immediate(immediate)
    }
    pub fn decode_instruction(instruction: Bytecode) -> Instruction {
        Instruction::new(
            Opcode::decode_opcode(instruction),
            Immediate::decode_immediate(instruction),
        )
    }
    pub fn print(&self) {
        println!("{self:?}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Opcode::*;

    #[test]
    fn test_instruction() {
        let instruction = Instruction::new(Pushc, 1);
        assert_eq!(instruction.opcode, Pushc);
        assert_eq!(instruction.immediate, 1);
    }

    #[test]
    fn test_encode_instruction() {
        assert_eq!(Instruction::encode_instruction(Pushc, 1), 0x01000001);
        assert_eq!(Instruction::encode_instruction(Pushc, -1), 0x01ffffff);
    }

    #[test]
    fn test_decode_instruction() {
        let decoded_instruction = Instruction::decode_instruction(0x01000001);
        assert_eq!(decoded_instruction.opcode, Pushc);
        assert_eq!(decoded_instruction.immediate, 1);
        let decoded_instruction = Instruction::decode_instruction(0x01ffffff);
        assert_eq!(decoded_instruction.opcode, Pushc);
        assert_eq!(decoded_instruction.immediate, -1);
    }

    #[test]
    fn test_print_instruction() {
        let instruction = Instruction::decode_instruction(0x01000001);
        instruction.print();
    }
}

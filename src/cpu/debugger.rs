use crate::{Instruction, NinjaVM, Opcode::Halt};

use std::io::{BufRead, Write};

impl<R, W> NinjaVM<R, W>
where
    R: BufRead + std::fmt::Debug,
    W: Write + std::fmt::Debug,
{
    pub fn debug_binary(&mut self, bin: &str) {
        let instructions = self.load_binary(bin);
        self.load_instructions(&instructions);
        self.instruction_cache.print();
        self.debug();
    }
    pub fn debug(&mut self) {
        self.init();
        let mut instruction = self.instruction_cache.register[self.instruction_cache.pc];
        let mut decoded_instruction = Instruction::decode_instruction(instruction);
        let mut opcode = decoded_instruction.opcode;
        while opcode != Halt {
            println!("---------------------");
            self.stack.print();
            self.sda.print();
            println!("Executing instruction");
            decoded_instruction.print();
            instruction = self.instruction_cache.register[self.instruction_cache.pc];
            decoded_instruction = Instruction::decode_instruction(instruction);
            opcode = decoded_instruction.opcode;
            self.instruction_cache.pc += 1;
            self.execute_instruction(instruction);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_debug_binary() {}
    #[test]
    fn test_debug() {}
}

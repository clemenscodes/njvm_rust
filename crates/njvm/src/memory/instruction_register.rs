use std::fmt::Debug;
use std::io::{StderrLock, StdinLock, StdoutLock};
use std::{
    cell::RefCell,
    io::{BufRead, Write},
    rc::Rc,
};

use crate::{
    cpu::{immediate::Immediate, instruction::Instruction, opcode::Opcode},
    io::InputOutput,
};

pub type Bytecode = u32;
pub type ProgramCounter = usize;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct InstructionRegister<
    R: BufRead + Debug,
    W: Write + Debug,
    E: Write + Debug,
> {
    pub io: Rc<RefCell<InputOutput<R, W, E>>>,
    pub pc: ProgramCounter,
    pub data: Vec<Bytecode>,
}

impl Default
    for InstructionRegister<StdinLock<'_>, StdoutLock<'_>, StderrLock<'_>>
{
    fn default() -> Self {
        let io = InputOutput::default();
        Self::new(Rc::new(RefCell::new(io)), 0, 0)
    }
}

impl<R: BufRead + Debug, W: Write + Debug, E: Write + Debug>
    InstructionRegister<R, W, E>
{
    pub fn new(
        io: Rc<RefCell<InputOutput<R, W, E>>>,
        size: usize,
        value: Bytecode,
    ) -> Self {
        let mut data = vec![];
        data.resize(size, value);
        InstructionRegister { io, pc: 0, data }
    }

    pub fn resize_data(&mut self, new_size: usize, value: Bytecode) {
        self.data.resize(new_size, value);
    }

    pub fn register_instruction(
        &mut self,
        opcode: Opcode,
        immediate: Immediate,
    ) {
        let instruction = Instruction::encode_instruction(opcode, immediate);
        self.data[self.pc] = instruction;
        self.pc += 1;
    }

    pub fn print(&mut self) {
        for pc in 0..self.data.len() {
            self.print_instruction(pc);
        }
    }

    pub fn print_instruction(&mut self, pc: usize) {
        let bytecode = self.data[pc];
        let instruction = Instruction::from(bytecode);
        let instruction = format!("{pc:04}: {instruction}");
        self.io.borrow().write_stdout(&instruction);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_memory() {
        let instruction_cache = InstructionRegister::default();
        assert_eq!(instruction_cache.pc, 0);
        assert_eq!(instruction_cache.data.len(), 0);
    }

    #[test]
    fn test_data_instruction() {
        let io = InputOutput::<StdinLock<'_>, StdoutLock<'_>, StderrLock<'_>>::default();
        let mut instruction_cache =
            InstructionRegister::new(Rc::new(RefCell::new(io)), 2, 0);
        instruction_cache
            .register_instruction(crate::cpu::opcode::Opcode::Pushc, 1);
        assert_eq!(instruction_cache.pc, 1);
        assert_eq!(instruction_cache.data[0], 0x01000001);
        instruction_cache
            .register_instruction(crate::cpu::opcode::Opcode::Pushc, 2);
        assert_eq!(instruction_cache.pc, 2);
        assert_eq!(instruction_cache.data[1], 0x01000002);
    }
}

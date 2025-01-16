use crate::cpu::{immediate::Immediate, instruction::Instruction, opcode::Opcode};

pub type Bytecode = u32;
pub type ProgramCounter = usize;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct InstructionRegister {
    pub pc: ProgramCounter,
    pub data: Vec<Bytecode>,
}

impl Default for InstructionRegister {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl InstructionRegister {
    pub fn new(size: usize, value: Bytecode) -> Self {
        let mut data = vec![];
        data.resize(size, value);
        InstructionRegister { pc: 0, data }
    }
    pub fn register_instruction(&mut self, opcode: Opcode, immediate: Immediate) {
        let instruction = Instruction::encode_instruction(opcode, immediate);
        self.data[self.pc] = instruction;
        self.pc += 1;
    }
    pub fn print(&mut self) {
        for i in 0..self.data.len() {
            self.print_instruction(i);
        }
    }
    pub fn print_instruction(&mut self, pc: usize) {
        use Opcode::*;

        let instruction = self.data[pc];
        let decoded = Instruction::decode_instruction(instruction);
        let opcode = decoded.opcode;
        let immediate = decoded.immediate;
        match opcode {
            Halt => println!("{pc:04}:\thalt"),
            Pushc => println!("{pc:04}:\tpushc\t{immediate}"),
            Add => println!("{pc:04}:\tadd"),
            Sub => println!("{pc:04}:\tsub"),
            Mul => println!("{pc:04}:\tmul"),
            Div => println!("{pc:04}:\tdiv"),
            Mod => println!("{pc:04}:\tmod"),
            Rdint => println!("{pc:04}:\trdint"),
            Wrint => println!("{pc:04}:\twrint"),
            Rdchr => println!("{pc:04}:\trdchr"),
            Wrchr => println!("{pc:04}:\twrchr"),
            Pushg => println!("{pc:04}:\tpushg\t{immediate}"),
            Popg => println!("{pc:04}:\tpopg\t{immediate}"),
            Asf => println!("{pc:04}:\tasf\t{immediate}"),
            Rsf => println!("{pc:04}:\trsf"),
            Pushl => println!("{pc:04}:\tpushl\t{immediate}"),
            Popl => println!("{pc:04}:\tpopl\t{immediate}"),
            Eq => println!("{pc:04}:\teq"),
            Ne => println!("{pc:04}:\tne"),
            Lt => println!("{pc:04}:\tlt"),
            Le => println!("{pc:04}:\tle"),
            Gt => println!("{pc:04}:\tgt"),
            Ge => println!("{pc:04}:\tge"),
            Jmp => println!("{pc:04}:\tjmp\t{immediate}"),
            Brf => println!("{pc:04}:\tbrf\t{immediate}"),
            Brt => println!("{pc:04}:\tbrt\t{immediate}"),
            Call => println!("{pc:04}:\tcall\t{immediate}"),
            Ret => println!("{pc:04}:\tret"),
            Drop => println!("{pc:04}:\tdrop\t{immediate}"),
            Pushr => println!("{pc:04}:\tpushr"),
            Popr => println!("{pc:04}:\tpopr"),
            Dup => println!("{pc:04}:\tdup"),
        }
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
        let mut instruction_cache = InstructionRegister::new(2, 0);
        instruction_cache.register_instruction(crate::cpu::opcode::Opcode::Pushc, 1);
        assert_eq!(instruction_cache.pc, 1);
        assert_eq!(instruction_cache.data[0], 0x01000001);
        instruction_cache.register_instruction(crate::cpu::opcode::Opcode::Pushc, 2);
        assert_eq!(instruction_cache.pc, 2);
        assert_eq!(instruction_cache.data[1], 0x01000002);
    }
}

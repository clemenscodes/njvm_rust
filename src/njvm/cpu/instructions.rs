#[macro_export]
macro_rules! immediate {
    ($e:expr) => {
        ($e) & 0x00FFFFFF
    };
}

#[macro_export]
macro_rules! sign_extend {
    ($e:expr) => {
        if ((($e) & 0x00800000) == 1) {
            ($e) | 0xFF000000
        } else {
            ($e)
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

impl Opcode {
    pub fn match_opcode(code: u8) -> Opcode {
        let opcode: Opcode;
        match code {
            0 => opcode = Opcode::Halt,
            1 => opcode = Opcode::Pushc,
            2 => opcode = Opcode::Add,
            3 => opcode = Opcode::Sub,
            4 => opcode = Opcode::Mul,
            5 => opcode = Opcode::Div,
            6 => opcode = Opcode::Mod,
            7 => opcode = Opcode::Rdint,
            8 => opcode = Opcode::Wrint,
            9 => opcode = Opcode::Rdchr,
            10 => opcode = Opcode::Wrchr,
            _ => panic!("Invalid opcode"),
        }
        opcode
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub immediate: u32,
}

impl Instruction {
    pub fn new(opcode: Opcode, immediate: u32) -> Self {
        Self { opcode, immediate }
    }
    pub fn encode_opcode(opcode: Opcode) -> u32 {
        let encoded_opcode = (opcode as u32) << 24;
        println!("OPCODE: {opcode:#?}");
        println!("OPCODE VALUE: {}", opcode as u32);
        println!("ENCODED OPCODE: {encoded_opcode:032b}");
        encoded_opcode
    }
    pub fn decode_instruction(bytecode: u32) -> Self {
        let instruction = Instruction::new(
            Opcode::match_opcode((bytecode << 24) as u8),
            sign_extend!(immediate!(bytecode)),
        );
        instruction.print();
        instruction
    }
    pub fn print(&self) {
        println!("{self:#?}")
    }
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, Opcode};
    fn testing() {
        Instruction::encode_opcode(Opcode::Pushc);
        // fn immediate(i: u32) -> u32 {
        //     i & 0x00FFFFFF
        // }
        // fn sign_extend(i: u32) -> u32 {
        //     println!("{i:032b}");
        //     let tmp = immediate(i);
        //     let tmp = tmp & 0x00800000;
        //     println!("{tmp:032b}");
        //     tmp
        // }
        // sign_extend(2);
        // let pushc_value = (Opcode::Pushc as u32) << 24;
        // let instruction = pushc_value | immediate(2);
        // println!("{pushc_value:032b}");
        // println!("{instruction:032b}");
    }
    #[test]
    fn test_display_binary() {
        testing()
    }
}

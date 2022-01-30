pub mod instructions;
pub use instructions::{Instruction, Opcode, Bytecode, Immediate};
pub mod worker;
pub use worker::Worker;
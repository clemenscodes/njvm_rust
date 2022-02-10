pub mod instruction;
pub use instruction::Instruction;
pub mod opcode;
pub use opcode::Opcode;
pub mod immediate;
pub mod processor;
pub use immediate::{Decoding, Encoding, Immediate};

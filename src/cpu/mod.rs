pub mod instruction;
pub use instruction::Instruction;
pub mod opcode;
pub use opcode::Opcode;
pub mod processor;
pub use processor::Processor;
pub mod immediate;
pub use immediate::{Decoding, Encoding, Immediate};

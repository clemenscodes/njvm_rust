pub mod stack;
pub use stack::Stack;
pub mod static_data_area;
pub use static_data_area::StaticDataArea;
pub mod instruction_cache;
pub use instruction_cache::{Bytecode, InstructionCache};

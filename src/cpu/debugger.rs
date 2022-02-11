use crate::NinjaVM;

use std::io::{BufRead, Write};

impl<R, W> NinjaVM<R, W>
where
    R: BufRead,
    W: Write,
{
    pub fn debug_binary(&mut self, bin: &str) {
        let instructions = self.load_binary(bin);
        self.load_instructions(&instructions);
        self.instruction_cache.print();
        self.debug();
    }
    pub fn debug(&mut self) {
        println!("Debugging");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_debug_binary() {}
    #[test]
    fn test_debug() {}
}

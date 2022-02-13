use crate::{fatal_error, NinjaVM};
use std::fmt::Debug;
use std::io::{BufRead, Write};

impl<R: BufRead + Debug, W: Write + Debug> NinjaVM<R, W> {
    pub fn debug(&mut self, bin: &str) {
        let instructions = self.load_binary(bin);
        self.load_instructions(&instructions);
        let code_size = self.ir.register.len();
        let data_size = self.sda.memory.len();
        println!("DEBUG: file '{bin}' loaded (code size = {code_size}, data size = {data_size})");
        self.init();
        self.prompt();
    }
    pub fn prompt(&mut self) {
        loop {
            self.print_next_instruction();
            println!("DEBUG: inspect, list, breakpoint, step, run, quit?");
            let mut input = String::new();
            if self.reader.read_line(&mut input).is_err() {
                fatal_error("Error: could not read line")
            }
            let input = input.trim();
            if let Some(input) = input.chars().next() {
                match input {
                    'i' => self.inspect(),
                    'l' => self.print_instructions(),
                    'b' => self.set_breakpoint(),
                    's' => self.step(),
                    'r' => self.work(),
                    'q' => self.halt(),
                    _ => continue,
                }
            } else {
                fatal_error("Error: could not read input")
            }
        }
    }
    pub fn inspect(&mut self) {
        println!("DEBUG: [inspect]: stack, data?");
        let mut input = String::new();
        if self.reader.read_line(&mut input).is_err() {
            fatal_error("Error: could not read input")
        }
        let input = input.trim();
        if let Some(input) = input.chars().next() {
            match input {
                's' => self.print_stack(),
                'd' => self.print_sda(),
                _ => {}
            }
        } else {
            fatal_error("Error: could not read input")
        }
    }
    pub fn print_stack(&mut self) {
        println!("------------------");
        self.stack.print();
        println!("------------------");
    }
    pub fn print_sda(&mut self) {
        println!("------------------");
        self.sda.print();
        println!("------------------");
    }
    pub fn print_instructions(&mut self) {
        println!("------------------");
        self.ir.print();
        println!("------------------");
    }
    pub fn print_next_instruction(&mut self) {
        self.ir.print_instruction(self.ir.pc);
    }
    pub fn step(&mut self) {
        let instruction = self.ir.register[self.ir.pc];
        self.ir.pc += 1;
        self.execute_instruction(instruction);
    }
    pub fn set_breakpoint(&mut self) {
        println!("Called set_breakpoint")
    }
}

#[cfg(test)]
mod tests {
    use crate::{InstructionRegister, NinjaVM, Opcode::*};
    use std::io::stdout;
    #[test]
    fn test_debug() {
        let input = b"quit";
        let mut vm = NinjaVM::new(&input[..], stdout());
        vm.debug("tests/data/a3/prog1.bin");
    }
    #[test]
    fn test_prompt() {}
    #[test]
    fn test_print_debug_info() {}
    #[test]
    fn test_print_stack() {}
    #[test]
    fn test_print_sda() {}
    #[test]
    fn test_print_instructions() {}
    #[test]
    fn test_print_next_instruction() {
        let mut vm = NinjaVM::default();
        vm.ir = InstructionRegister::new(3, 0);
        vm.ir.register_instruction(Pushc, 1);
        vm.ir.register_instruction(Pushc, 2);
        vm.ir.register_instruction(Add, 0);
        vm.init();
        vm.print_next_instruction();
        vm.ir.pc += 1;
        vm.print_next_instruction();
        vm.ir.pc += 1;
        vm.print_next_instruction();
    }
    #[test]
    fn test_step() {}
    #[test]
    fn test_run() {}
    #[test]
    fn test_set_breakpoint() {}
    #[test]
    fn test_quit() {}
}

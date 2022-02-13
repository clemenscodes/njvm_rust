use crate::{fatal_error, NinjaVM};
use std::fmt::Debug;
use std::io::{BufRead, Write};

impl<R: BufRead + Debug, W: Write + Debug> NinjaVM<R, W> {
    pub fn debug(&mut self, bin: &str) {
        let instructions = self.load_binary(bin);
        self.load_instructions(&instructions);
        let code_size = self.ir.data.len();
        let data_size = self.sda.data.len();
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
                    'r' => self.run(),
                    'q' => self.halt(),
                    _ => continue,
                }
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
        let instruction = self.ir.data[self.ir.pc];
        self.ir.pc += 1;
        self.execute_instruction(instruction);
    }
    pub fn run(&mut self) {
        loop {
            if let Some(bp) = self.bp {
                if bp == self.ir.pc {
                    self.prompt()
                }
            }
            self.step()
        }
    }
    pub fn set_breakpoint(&mut self) {
        if let Some(bp) = self.bp {
            println!("DEBUG [breakpoint]: breakpoint is set at {bp}");
        } else {
            println!("DEBUG [breakpoint]: cleared")
        }
        println!("DEBUG [breakpoint]: address to set, -1 to clear, <ret> for no change?");
        let mut input = String::new();
        if self.reader.read_line(&mut input).is_err() {
            fatal_error("Error: could not read input")
        }
        let bp: isize = match String::from(input.trim()).parse() {
            Ok(bp) => bp,
            Err(_) => return,
        };
        if bp < -1 {
            return;
        }
        match bp {
            -1 => {
                self.bp = None;
                println!("DEBUG [breakpoint]: now cleared");
            }
            _ => {
                let bp = bp as usize;
                self.bp = Some(bp);
                println!("DEBUG [breakpoint]: now set at {bp}");
            }
        }
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
        vm.ir.data_instruction(Pushc, 1);
        vm.ir.data_instruction(Pushc, 2);
        vm.ir.data_instruction(Add, 0);
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

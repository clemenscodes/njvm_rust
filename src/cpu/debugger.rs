use crate::{fatal_error, Instruction, NinjaVM, Opcode::*};

use std::io::{BufRead, Write};

impl<R, W> NinjaVM<R, W>
where
    R: BufRead + std::fmt::Debug,
    W: Write + std::fmt::Debug,
{
    pub fn debug(&mut self, bin: &str) {
        let instructions = self.load_binary(bin);
        self.load_instructions(&instructions);
        self.print_debug_info(bin);
        self.init();
        self.prompt();
    }
    pub fn print_debug_info(&mut self, bin: &str) {
        let code_size = self.instruction_cache.register.len();
        let data_size = self.sda.memory.len();
        println!("DEBUG: file '{bin}' loaded (code size = {code_size}, data size = {data_size})");
    }
    pub fn prompt(&mut self) {
        loop {
            self.print_next_instruction();
            println!("DEBUG: inspect, list, breakpoint, step, run, quit?");
            let mut input = String::new();
            if self.reader.read_line(&mut input).is_ok() {
                let input = input.trim();
                if input.starts_with('i') {
                    println!("DEBUG: [inspect]: stack, data?");
                    let mut input = String::new();
                    if self.reader.read_line(&mut input).is_ok() {
                        let input = input.trim();
                        if input.starts_with('s') {
                            self.print_stack();
                            continue;
                        }
                        if input.starts_with('d') {
                            self.print_sda();
                            continue;
                        }
                        continue;
                    } else {
                        fatal_error("Error: could not read input")
                    }
                }
                if input.starts_with('l') {
                    self.print_instructions();
                    continue;
                }
                if input.starts_with('b') {
                    self.set_breakpoint();
                    continue;
                }
                if input.starts_with('s') {
                    self.step();
                    continue;
                }
                if input.starts_with('r') {
                    self.run();
                    continue;
                }
                if input.starts_with('q') {
                    self.quit();
                    break;
                }
                continue;
            } else {
                fatal_error("Error: could not read character")
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
        self.instruction_cache.print();
        println!("------------------");
    }
    pub fn print_next_instruction(&mut self) {
        self.instruction_cache.print_instruction(self.instruction_cache.pc);
    }
    pub fn step(&mut self) {
        let instruction = self.instruction_cache.register[self.instruction_cache.pc];
        let decoded = Instruction::decode_instruction(instruction);
        if decoded.opcode == Halt {
            self.execute_instruction(instruction);
        }
        self.instruction_cache.pc += 1;
        self.execute_instruction(instruction);
    }
    pub fn run(&mut self) {
        let mut instruction = self.instruction_cache.register[self.instruction_cache.pc];
        let mut decoded_instruction = Instruction::decode_instruction(instruction);
        let mut opcode = decoded_instruction.opcode;
        while opcode != Halt {
            instruction = self.instruction_cache.register[self.instruction_cache.pc];
            decoded_instruction = Instruction::decode_instruction(instruction);
            opcode = decoded_instruction.opcode;
            self.instruction_cache.pc += 1;
            self.execute_instruction(instruction);
        }
    }
    pub fn set_breakpoint(&mut self) {
        println!("Called set_breakpoint")
    }
    pub fn quit(&mut self) {
        self.halt();
    }
}

#[cfg(test)]
mod tests {
    use crate::{InstructionCache, NinjaVM, Opcode::*};
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
        vm.instruction_cache = InstructionCache::new(3, 0);
        vm.instruction_cache.register_instruction(Pushc, 1);
        vm.instruction_cache.register_instruction(Pushc, 2);
        vm.instruction_cache.register_instruction(Add, 0);
        vm.init();
        vm.print_next_instruction();
        vm.instruction_cache.pc += 1;
        vm.print_next_instruction();
        vm.instruction_cache.pc += 1;
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

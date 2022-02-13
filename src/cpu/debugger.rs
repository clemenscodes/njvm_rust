use crate::{fatal_error, Instruction, NinjaVM, Opcode::*};

use std::io::{BufRead, Write};

pub trait Debugger {
    fn debug(&mut self, bin: &str);
    fn prompt(&mut self);
    fn print_debug_info(&mut self, bin: &str);
    fn print_stack(&mut self);
    fn print_sda(&mut self);
    fn print_instructions(&mut self);
    fn print_next_instruction(&mut self);
    fn step(&mut self);
    fn run(&mut self);
    fn set_breakpoint(&mut self);
    fn quit(&mut self);
}

impl<R, W> Debugger for NinjaVM<R, W>
where
    R: BufRead + std::fmt::Debug,
    W: Write + std::fmt::Debug,
{
    fn debug(&mut self, bin: &str) {
        let instructions = self.load_binary(bin);
        self.load_instructions(&instructions);
        self.print_debug_info(bin);
        self.init();
        self.print_next_instruction();
        self.prompt();
    }
    fn print_debug_info(&mut self, bin: &str) {
        let code_size = self.instruction_cache.register.len();
        let data_size = self.sda.memory.len();
        println!("DEBUG: file '{bin}' loaded (code size = {code_size}, data size = {data_size})");
    }
    fn prompt(&mut self) {
        loop {
            println!("DEBUG: inspect, list, breakpoint, step, run quit?");
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
    fn print_stack(&mut self) {
        self.stack.print();
    }
    fn print_sda(&mut self) {
        self.sda.print();
    }
    fn print_instructions(&mut self) {
        self.instruction_cache.print();
    }
    fn print_next_instruction(&mut self) {
        let pc = self.instruction_cache.pc;
        let next_instruction = self.instruction_cache.register[pc];
        let decoded_instruction = Instruction::decode_instruction(next_instruction);
        let opcode = decoded_instruction.opcode;
        let immediate = decoded_instruction.immediate;
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
        }
    }
    fn step(&mut self) {
        println!("Called step")
    }
    fn run(&mut self) {
        println!("Called run")
    }
    fn set_breakpoint(&mut self) {
        println!("Called set_breakpoint")
    }
    fn quit(&mut self) {
        println!("Called quit")
    }
}

#[cfg(test)]
mod tests {
    use crate::{Debugger, InstructionCache, NinjaVM, Opcode::*};
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

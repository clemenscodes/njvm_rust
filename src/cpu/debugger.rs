use std::fmt::Debug;
use std::io::{BufRead, Write};

use crate::NinjaVM;

impl<R: BufRead + Debug, W: Write + Debug, E: Write + Debug> NinjaVM<R, W, E> {
    pub fn debug(&mut self, bin: &str) {
        let instructions = self.load_binary(bin);
        self.load_instructions(&instructions);
        let code_size = self.ir.data.len();
        let data_size = self.sda.data.len();
        let message = format!("DEBUG: file '{bin}' loaded (code size = {code_size}, data size = {data_size})\n");
        self.io_borrow().write_stdout(&message);
        self.init();
        self.prompt();
    }

    pub fn test_debug(&mut self, bin: &str) {
        let instructions = self.load_test_binary(bin);
        self.load_instructions(&instructions);
        let code_size = self.ir.data.len();
        let data_size = self.sda.data.len();
        let message = format!("DEBUG: file '{bin}' loaded (code size = {code_size}, data size = {data_size})\n");
        self.io_borrow().write_stdout(&message);
        self.init();
        self.prompt();
    }

    pub fn prompt(&mut self) {
        loop {
            if self.ir.pc == self.ir.data.len() {
                break;
            }
            self.print_next_instruction();
            self.io_borrow().write_stdout(
                "DEBUG: inspect, list, breakpoint, step, run, quit?\n",
            );
            let mut input = String::new();
            if self
                .io_borrow()
                .stdin_borrow_mut()
                .read_line(&mut input)
                .is_err()
            {
                self.io.borrow().fatal_error("Error: could not read line\n")
            }
            let input = input.trim();
            if let Some(input) = input.chars().next() {
                match input {
                    'i' => self.inspect(),
                    'l' => self.print_ir(),
                    'b' => self.set_breakpoint(),
                    's' => self.step(),
                    'r' => {
                        self.run();
                        break;
                    }
                    'q' => {
                        self.halt();
                        break;
                    }
                    _ => continue,
                }
            }
        }
    }

    pub fn inspect(&mut self) {
        self.io_borrow()
            .write_stdout("DEBUG: [inspect]: stack, data?\n");
        let mut input = String::new();
        if self
            .io_borrow()
            .stdin_borrow_mut()
            .read_line(&mut input)
            .is_err()
        {
            self.io_borrow()
                .fatal_error("Error: could not read input\n")
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
    pub fn step(&mut self) {
        let instruction = self.ir.data[self.ir.pc];
        self.ir.pc += 1;
        self.execute_instruction(instruction);
    }
    pub fn run(&mut self) {
        loop {
            if self.ir.pc == self.ir.data.len() {
                break;
            }
            if let Some(bp) = self.bp {
                if bp == self.ir.pc {
                    self.bp = None;
                    self.io_borrow()
                        .write_stdout("DEBUG [breakpoint]: cleared\n");
                    self.prompt();
                    break;
                }
            }
            self.step()
        }
    }
    pub fn set_breakpoint(&mut self) {
        if let Some(bp) = self.bp {
            let message =
                format!("DEBUG [breakpoint]: breakpoint is set at {bp}\n");
            self.io_borrow().write_stdout(&message);
        } else {
            self.io_borrow()
                .write_stdout("DEBUG [breakpoint]: cleared\n");
        }
        self.io_borrow().write_stdout("DEBUG [breakpoint]: address to set, -1 to clear, <ret> for no change?\n");
        let mut input = String::new();
        if self
            .io_borrow()
            .stdin_borrow_mut()
            .read_line(&mut input)
            .is_err()
        {
            self.io_borrow()
                .fatal_error("Error: could not read input\n")
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
                self.io_borrow()
                    .write_stdout("DEBUG [breakpoint]: now cleared\n");
            }
            _ => {
                let bp = bp as usize;
                self.bp = Some(bp);
                let message = format!("DEBUG [breakpoint]: now set at {bp}\n");
                self.io_borrow().write_stdout(&message);
            }
        }
    }

    pub fn print_next_instruction(&mut self) {
        self.ir.print_instruction(self.ir.pc);
    }

    pub fn print_stack(&mut self) {
        self.io_borrow()
            .write_stdout("-----------------------------\n");
        self.stack.print();
        self.io_borrow()
            .write_stdout("-----------------------------\n");
    }

    pub fn print_sda(&mut self) {
        self.io_borrow().write_stdout("------------------\n");
        self.sda.print();
        self.io_borrow().write_stdout("------------------\n");
    }

    pub fn print_ir(&mut self) {
        self.io_borrow().write_stdout("------------------\n");
        self.ir.print();
        self.io_borrow().write_stdout("------------------\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::cpu::opcode::Opcode::*;
    use crate::io::InputOutput;

    #[test]
    fn test_prompt() {
        let input = b"s\n8\nq\n";
        let mut vm = NinjaVM::new(InputOutput::new(
            &input[..],
            std::io::stdout(),
            std::io::stderr(),
        ));
        let instructions = vm.load_test_binary("assets/a3/prog1.bin");
        vm.load_instructions(&instructions);
        vm.init();
        vm.prompt();
    }

    #[test]
    fn test_step() {
        let input = b"9\n";
        let mut vm = NinjaVM::new(InputOutput::new(
            &input[..],
            std::io::stdout(),
            std::io::stderr(),
        ));
        let instructions = vm.load_test_binary("assets/a3/prog1.bin");
        vm.load_instructions(&instructions);
        vm.init();
        vm.step();
        vm.stack.print();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.fp, 0);
        assert_eq!(vm.stack.data.len(), 1);
        assert_eq!(vm.stack.data[0], 9)
    }

    #[test]
    fn test_run() {
        let input = b"b\n23\nr\n8\n12\nq\n";
        let mut vm = NinjaVM::new(InputOutput::new(
            &input[..],
            std::io::stdout(),
            std::io::stderr(),
        ));
        vm.test_debug("assets/a3/prog1.bin");
        assert_eq!(vm.ir.data.len(), 27);
        assert_eq!(vm.sda.data.len(), 2);
        assert_eq!(vm.sda.data[0], 4);
        assert_eq!(vm.sda.data[1], 4);
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.fp, 0);
        assert_eq!(vm.stack.data.len(), 1);
    }

    #[test]
    fn test_set_breakpoint() {
        let input = b"b\n23\nq\nb\n-1\nq\n";
        let mut vm = NinjaVM::new(InputOutput::new(
            &input[..],
            std::io::stdout(),
            std::io::stderr(),
        ));
        vm.test_debug("assets/a3/prog1.bin");
        assert_eq!(vm.bp, Some(23));
        vm.test_debug("assets/a3/prog1.bin");
        assert_eq!(vm.bp, None);
    }

    #[test]
    fn test_list_ir() {
        let input = b"l\nq\n";
        let mut vm = NinjaVM::new(InputOutput::new(
            &input[..],
            std::io::stdout(),
            std::io::stderr(),
        ));
        vm.test_debug("assets/a3/prog1.bin");
    }

    #[test]
    fn test_debugger_breaks_at_breakpoint() {
        let input = b"b\n5\nr\n8\n12\nq\nb\n23\nr\nq\n";
        let mut vm = NinjaVM::new(InputOutput::new(
            &input[..],
            std::io::stdout(),
            std::io::stderr(),
        ));
        vm.test_debug("assets/a3/prog1.bin");
        assert_eq!(vm.ir.pc, 5);
        assert_eq!(vm.bp, None);
        vm.prompt();
        assert_eq!(vm.stack.data.len(), 1);
        assert_eq!(vm.sda.data.len(), 2);
        assert_eq!(vm.sda.data[0], 4);
        assert_eq!(vm.sda.data[1], 4);
    }

    #[test]
    fn test_print_next_instruction() {
        let mut vm = NinjaVM::default();
        vm.ir.resize_data(3, 0);
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
}

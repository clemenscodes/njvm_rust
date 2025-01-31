pub mod bigint;
pub mod cpu;
pub mod io;
pub mod memory;

use std::cell::RefCell;
use std::fmt::Debug;
use std::io::{BufRead, StderrLock, StdinLock, StdoutLock, Write};
use std::rc::Rc;

use cpu::immediate::Immediate;
use cpu::instruction::Instruction;
use io::InputOutput;
use memory::instruction_register::{Bytecode, InstructionRegister};
use memory::stack::Stack;
use memory::static_data_area::StaticDataArea;

pub const VERSION: u8 = 4;

pub type Breakpoint = usize;
pub type ReturnValueRegister = Immediate;

#[derive(Debug, Clone)]
pub struct NinjaVM<R: BufRead + Debug, W: Write + Debug, E: Write + Debug> {
    io: Rc<RefCell<InputOutput<R, W, E>>>,
    stack: Stack<R, W, E, Immediate>,
    ir: InstructionRegister<R, W, E>,
    sda: StaticDataArea<R, W, E, Immediate>,
    bp: Option<Breakpoint>,
    rv: Option<ReturnValueRegister>,
}

impl Default for NinjaVM<StdinLock<'_>, StdoutLock<'_>, StderrLock<'_>> {
    fn default() -> Self {
        NinjaVM::new(InputOutput::default())
    }
}

impl<R: BufRead + Debug, W: Write + Debug, E: Write + Debug> NinjaVM<R, W, E> {
    pub fn start(args: Vec<String>) {
        let mut vm = NinjaVM::default();

        if args.is_empty() {
            vm.io_borrow()
                .fatal_error("Error: no code file specified\n");
        }

        let mut debug_mode = false;
        let mut file: Option<String> = None;

        for arg in args {
            match arg.as_str() {
                "--help" => {
                    vm.help();
                    return;
                }
                "--version" => {
                    vm.version();
                    return;
                }
                "--debug" => {
                    if debug_mode {
                        vm.io_borrow()
                            .fatal_error("Error: duplicate '--debug' flag\n");
                    }
                    debug_mode = true;
                }
                _ if arg.starts_with('-') => vm.io_borrow().unknown_arg(&arg),
                _ => {
                    if file.is_some() {
                        vm.io_borrow().fatal_error(
                            "Error: more than one code file specified\n",
                        );
                    }
                    file = Some(arg);
                }
            }
        }

        let file = file.unwrap_or_else(|| {
            vm.io_borrow()
                .fatal_error("Error: no code file specified\n")
        });

        if debug_mode {
            vm.debug(&file);
        } else {
            vm.execute_binary(&file);
        }
    }

    pub fn new(io: InputOutput<R, W, E>) -> Self {
        let io = Rc::new(RefCell::new(io));

        Self {
            io: io.clone(),
            stack: Stack::new(io.clone()),
            ir: InstructionRegister::new(io.clone(), 0, 0),
            sda: StaticDataArea::new(io.clone(), 0, 0),
            bp: None,
            rv: None,
        }
    }

    pub fn execute_instruction(&mut self, bytecode: Bytecode) {
        use cpu::opcode::Opcode::*;

        let instruction = Instruction::from(bytecode);
        let immediate = instruction.immediate;

        match instruction.opcode {
            Halt => self.halt(),
            Pushc => self.pushc(immediate),
            Add => self.add(),
            Sub => self.sub(),
            Mul => self.mul(),
            Div => self.div(),
            Mod => self.modulo(),
            Rdint => self.rdint(),
            Wrint => self.wrint(),
            Rdchr => self.rdchr(),
            Wrchr => self.wrchr(),
            Pushg => self.pushg(immediate),
            Popg => self.popg(immediate),
            Asf => self.asf(immediate),
            Rsf => self.rsf(),
            Pushl => self.pushl(immediate),
            Popl => self.popl(immediate),
            Eq => self.eq(),
            Ne => self.ne(),
            Lt => self.lt(),
            Le => self.le(),
            Gt => self.gt(),
            Ge => self.ge(),
            Jmp => self.jmp(immediate),
            Brf => self.brf(immediate),
            Brt => self.brt(immediate),
            Call => self.call(immediate),
            Ret => self.ret(),
            Drop => self.drop(immediate),
            Pushr => self.pushr(),
            Popr => self.popr(),
            Dup => self.dup(),
        }
    }

    pub fn work(&mut self) {
        loop {
            let bytecode = self.ir.data[self.ir.pc];
            let decoded = Instruction::from(bytecode);
            let opcode = decoded.opcode;
            self.ir.pc += 1;
            self.execute_instruction(bytecode);
            if opcode == cpu::opcode::Opcode::Halt {
                break;
            }
        }
    }

    pub fn execute_binary(&mut self, bin: &str) {
        let instructions = self.load_binary(bin);
        self.load_instructions(&instructions);
        self.init();
        self.work();
    }

    pub fn load_binary(&mut self, arg: &str) -> Vec<u8> {
        self.io_borrow().verify_arg(arg);
        let mut file = self.io_borrow().read_file(arg);
        let instructions = self.io_borrow().split_file_metadata(&mut file);
        self.io_borrow().check_ninja_format(&file, arg);
        self.io_borrow().check_ninja_version(&file);
        let variable_count = self.io_borrow().check_variables(&file);
        let instruction_count = self.io_borrow().check_instructions(&file);
        self.sda.data.resize(variable_count, 0);
        self.ir.data.resize(instruction_count, 0);
        instructions
    }

    pub fn load_test_binary(&mut self, arg: &str) -> Vec<u8> {
        self.io_borrow().verify_arg(arg);
        let mut file = self.io_borrow().read_file(arg);
        let instructions = self.io_borrow().split_file_metadata(&mut file);
        self.io_borrow().check_ninja_format(&file, arg);
        self.io_borrow().set_ninja_version(&mut file);
        let variable_count = self.io_borrow().check_variables(&file);
        let instruction_count = self.io_borrow().check_instructions(&file);
        self.sda.data.resize(variable_count, 0);
        self.ir.data.resize(instruction_count, 0);
        instructions
    }

    pub fn load_instructions(&mut self, instructions: &[u8]) {
        instructions.chunks(4).for_each(|c| {
            let bytecode = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
            let instruction = Instruction::from(bytecode);
            let opcode = instruction.opcode;
            let immediate = instruction.immediate;
            self.ir.register_instruction(opcode, immediate);
        });
    }

    pub fn load(&mut self, bin: &str) {
        let instructions = self.load_binary(bin);
        self.load_instructions(&instructions)
    }

    pub fn init(&mut self) {
        self.io_borrow()
            .write_stdout("Ninja Virtual Machine started\n");
        self.ir.pc = 0;
    }

    fn help(&self) {
        self.io_borrow()
            .write_stdout("usage: ./njvm [options] <code file>\n");
        self.io_borrow().write_stdout("Options:\n");
        self.io_borrow().write_stdout(
            "  --debug          start virtual machine in debug mode\n",
        );
        self.io_borrow()
            .write_stdout("  --version        show version and exit\n");
        self.io_borrow()
            .write_stdout("  --help           show this help and exit\n");
    }

    fn version(&self) {
        self.io_borrow().write_stdout("Ninja Virtual Machine version {VERSION} (compiled Sep 23 2015, 10:36:52)\n",);
    }

    pub fn io_borrow(&self) -> std::cell::Ref<'_, InputOutput<R, W, E>> {
        self.io.borrow()
    }

    pub fn io_borrow_mut(&self) -> std::cell::RefMut<'_, InputOutput<R, W, E>> {
        self.io.borrow_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpu::opcode::Opcode::*;

    #[test]
    fn test_ninja_vm() {
        let stdin = b"";
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let vm = NinjaVM::new(InputOutput::new(
            &stdin[..],
            &mut stdout,
            &mut stderr,
        ));
        assert_eq!(vm.stack.sp, 0);
        assert_eq!(vm.stack.data.len(), 0);
        assert_eq!(vm.ir.pc, 0);
        assert_eq!(vm.ir.data.len(), 0);
    }

    #[test]
    fn test_work() {
        let stdin = b"";
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let mut vm = NinjaVM::new(InputOutput::new(
            &stdin[..],
            &mut stdout,
            &mut stderr,
        ));
        vm.ir.resize_data(3, 0);
        vm.ir.register_instruction(Pushc, 1);
        vm.ir.register_instruction(Pushc, 2);
        vm.ir.register_instruction(Halt, 0);
        vm.init();
        vm.work();
        assert_eq!(vm.stack.data.len(), 2);
    }

    #[test]
    fn test_execute_instruction() {
        let stdin = b"";
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let mut vm = NinjaVM::new(InputOutput::new(
            &stdin[..],
            &mut stdout,
            &mut stderr,
        ));
        let instruction = Instruction::encode_instruction(Pushc, 1);
        vm.execute_instruction(instruction);
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.data[0], 1);
    }

    #[test]
    fn test_load_instruction() {
        let stdin = b"";
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let mut vm = NinjaVM::new(InputOutput::new(
            &stdin[..],
            &mut stdout,
            &mut stderr,
        ));
        let instructions = Vec::new();
        vm.load_instructions(&instructions);
    }

    #[test]
    fn test_prog_a4_02() {
        let stdin = b"";
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let mut vm = NinjaVM::new(InputOutput::new(
            &stdin[..],
            &mut stdout,
            &mut stderr,
        ));
        let instructions = vm.load_test_binary("assets/a4/prog02.bin");
        vm.load_instructions(&instructions);
        vm.init();
        vm.work();
        let output = String::from_utf8(stdout).unwrap();
        let expected = r#"Ninja Virtual Machine started
11
22
33
Ninja Virtual Machine stopped
"#;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_prog_a4_12() {
        let stdin = b"5\n";
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let mut vm = NinjaVM::new(InputOutput::new(
            &stdin[..],
            &mut stdout,
            &mut stderr,
        ));
        let instructions = vm.load_test_binary("assets/a4/prog12.bin");
        vm.load_instructions(&instructions);
        vm.init();
        vm.work();
        let output = String::from_utf8(stdout).unwrap();
        let expected = r#"Ninja Virtual Machine started
5! = 120
Ninja Virtual Machine stopped
"#;
        assert_eq!(output, expected);
    }
}

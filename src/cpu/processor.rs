use crate::{fatal_error, Bytecode, Immediate, InstructionCache, Stack, StaticDataArea};
use std::io::{BufRead, Write};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Processor<R, W> {
    pub stack: Stack<Immediate>,
    pub instruction_cache: InstructionCache<Bytecode>,
    pub sda: StaticDataArea<Immediate>,
    reader: R,
    writer: W,
}

impl<R, W> Processor<R, W>
where
    R: BufRead,
    W: Write,
{
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            stack: Stack::default(),
            instruction_cache: InstructionCache::default(),
            sda: StaticDataArea::default(),
            reader,
            writer,
        }
    }
    pub fn halt(&self) {
        println!("Ninja Virtual Machine stopped");
    }
    pub fn pushc(&mut self, immediate: Immediate) {
        self.stack.push(immediate);
    }
    pub fn add(&mut self) {
        let n2 = self.stack.pop();
        let n1 = self.stack.pop();
        self.stack.push(n1 + n2);
    }
    pub fn sub(&mut self) {
        let n2 = self.stack.pop();
        let n1 = self.stack.pop();
        self.stack.push(n1 - n2);
    }
    pub fn mul(&mut self) {
        let n2 = self.stack.pop();
        let n1 = self.stack.pop();
        self.stack.push(n1 * n2);
    }
    pub fn div(&mut self) {
        let n2 = self.stack.pop();
        let n1 = self.stack.pop();
        if n2 == 0 {
            fatal_error("Division by zero error");
        }
        self.stack.push(n1 / n2);
    }
    pub fn modulo(&mut self) {
        let n2 = self.stack.pop();
        let n1 = self.stack.pop();
        if n2 == 0 {
            fatal_error("Division by zero error");
        }
        self.stack.push(n1 % n2);
    }
    pub fn rdint(&mut self) {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed to read line");
        let immediate: Immediate = input.trim().parse::<i32>().expect("Input not an integer");
        self.stack.push(immediate)
    }
    pub fn wrint(&mut self) {
        write!(self.writer, "{}", self.stack.pop()).expect("Unable to write")
    }
    pub fn rdchr(&mut self) {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed to read line");
        let immediate = input.trim().chars().next().expect("Failed to read character") as Immediate;
        self.stack.push(immediate)
    }
    pub fn wrchr(&mut self) {
        let character = self.stack.pop() as u8 as char;
        write!(self.writer, "{character}").expect("Unable to write")
    }
    pub fn pushg(&mut self, immediate: Immediate) {
        println!("Called pushg with immediate {immediate}");
    }
    pub fn popg(&mut self, immediate: Immediate) {
        println!("Called popg with immediate {immediate}");
    }
    pub fn asf(&mut self, immediate: Immediate) {
        println!("Called asf with immediate {immediate}");
    }
    pub fn rsf(&mut self) {
        println!("Called rsf");
    }
    pub fn pushl(&mut self, immediate: Immediate) {
        println!("Called pushl with immediate {immediate}");
    }
    pub fn popl(&mut self, immediate: Immediate) {
        println!("Called popl with immediate {immediate}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{stdin, stdout};
    #[test]
    fn test_pushc() {
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        cpu.pushc(2);
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], 2);
    }
    #[test]
    fn test_add() {
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        cpu.pushc(-1);
        cpu.pushc(2);
        cpu.add();
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], 1);
    }
    #[test]
    fn test_sub() {
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        cpu.pushc(1);
        cpu.pushc(2);
        cpu.sub();
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], -1);
    }
    #[test]
    fn test_mul() {
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        cpu.pushc(-1);
        cpu.pushc(-2);
        cpu.mul();
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], 2);
    }
    #[test]
    fn test_div() {
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        cpu.pushc(-7);
        cpu.pushc(-2);
        cpu.div();
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], 3);
        cpu.pushc(-3);
        cpu.div();
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], -1);
    }
    #[test]
    #[should_panic(expected = "Division by zero error")]
    fn test_division_by_zero_should_fail() {
        std::panic::set_hook(Box::new(|_| {}));
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        cpu.pushc(-2);
        cpu.pushc(4);
        cpu.pushc(-4);
        cpu.add();
        cpu.div();
    }
    #[test]
    fn test_modulo() {
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        cpu.pushc(-9);
        cpu.pushc(4);
        cpu.modulo();
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], -1);
    }
    #[test]
    #[should_panic(expected = "Division by zero error")]
    fn test_modulo_with_zero_should_fail() {
        std::panic::set_hook(Box::new(|_| {}));
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        cpu.pushc(-2);
        cpu.pushc(4);
        cpu.pushc(-4);
        cpu.add();
        cpu.modulo();
    }
    #[test]
    fn test_rdint() {
        let input = b"1";
        let mut cpu = Processor::new(&input[..], stdout());
        cpu.rdint();
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], 1)
    }
    #[test]
    fn test_wrint() {
        let stdin = stdin();
        let mut output = Vec::new();
        let mut cpu = Processor::new(stdin.lock(), &mut output);
        let immediate: Immediate = 42;
        cpu.pushc(immediate);
        cpu.wrint();
        let output = String::from_utf8(output).expect("Not utf-8");
        assert_eq!(output, String::from("42"));
    }
    #[test]
    fn test_rdchr() {
        let input = b"1";
        let mut cpu = Processor::new(&input[..], stdout());
        cpu.rdchr();
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], 49)
    }
    #[test]
    fn test_wrchr() {
        let stdin = stdin();
        let mut output = Vec::new();
        let mut cpu = Processor::new(stdin.lock(), &mut output);
        let immediate: Immediate = '1'.to_ascii_lowercase() as i32;
        cpu.pushc(immediate);
        cpu.wrchr();
        let output = String::from_utf8(output).expect("Not utf-8");
        assert_eq!(output, String::from("1"));
    }
    #[test]
    fn test_pushg() {
        let stdin = stdin();
        let mut cpu = Processor::new(stdin.lock(), stdout());
        let immediate: Immediate = 5;
        cpu.pushg(immediate);
        assert_eq!(cpu.stack.sp, 1);
        assert_eq!(cpu.stack.memory[0], 1);
    }
    #[test]
    fn test_popg() {
    }#[test]
    fn test_asf() {
    }#[test]
    fn test_rsf() {
    }#[test]
    fn test_pushl() {
    }#[test]
    fn test_popl() {
    }
}

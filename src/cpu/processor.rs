use crate::{fatal_error, Immediate, NinjaVM};
use std::io::{BufRead, Write};

impl<R, W> NinjaVM<R, W>
where
    R: BufRead,
    W: Write,
{
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
        self.stack.push(self.sda.memory[immediate as usize]);
    }
    pub fn popg(&mut self, immediate: Immediate) {
        self.sda.memory[immediate as usize] = self.stack.pop();
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
    use crate::{Immediate, NinjaVM, StaticDataArea};
    use std::io::{stdin, stdout};
    #[test]
    fn test_pushc() {
        let mut vm = NinjaVM::default();
        vm.pushc(2);
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], 2);
    }
    #[test]
    fn test_add() {
        let mut vm = NinjaVM::default();
        vm.pushc(-1);
        vm.pushc(2);
        vm.add();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], 1);
    }
    #[test]
    fn test_sub() {
        let mut vm = NinjaVM::default();
        vm.pushc(1);
        vm.pushc(2);
        vm.sub();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], -1);
    }
    #[test]
    fn test_mul() {
        let mut vm = NinjaVM::default();
        vm.pushc(-1);
        vm.pushc(-2);
        vm.mul();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], 2);
    }
    #[test]
    fn test_div() {
        let mut vm = NinjaVM::default();
        vm.pushc(-7);
        vm.pushc(-2);
        vm.div();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], 3);
        vm.pushc(-3);
        vm.div();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], -1);
    }
    #[test]
    #[should_panic(expected = "Division by zero error")]
    fn test_division_by_zero_should_fail() {
        std::panic::set_hook(Box::new(|_| {}));
        let mut vm = NinjaVM::default();
        vm.pushc(-2);
        vm.pushc(4);
        vm.pushc(-4);
        vm.add();
        vm.div();
    }
    #[test]
    fn test_modulo() {
        let mut vm = NinjaVM::default();
        vm.pushc(-9);
        vm.pushc(4);
        vm.modulo();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], -1);
    }
    #[test]
    #[should_panic(expected = "Division by zero error")]
    fn test_modulo_with_zero_should_fail() {
        std::panic::set_hook(Box::new(|_| {}));
        let mut vm = NinjaVM::default();
        vm.pushc(-2);
        vm.pushc(4);
        vm.pushc(-4);
        vm.add();
        vm.modulo();
    }
    #[test]
    fn test_rdint() {
        let input = b"1";
        let mut vm = NinjaVM::new(&input[..], stdout());
        vm.rdint();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], 1)
    }
    #[test]
    fn test_wrint() {
        let stdin = stdin();
        let mut output = Vec::new();
        let mut vm = NinjaVM::new(stdin.lock(), &mut output);
        let immediate: Immediate = 42;
        vm.pushc(immediate);
        vm.wrint();
        let output = String::from_utf8(output).expect("Not utf-8");
        assert_eq!(output, String::from("42"));
    }
    #[test]
    fn test_rdchr() {
        let input = b"1";
        let mut vm = NinjaVM::new(&input[..], stdout());
        vm.rdchr();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.memory[0], 49)
    }
    #[test]
    fn test_wrchr() {
        let stdin = stdin();
        let mut output = Vec::new();
        let mut vm = NinjaVM::new(stdin.lock(), &mut output);
        let immediate: Immediate = '1'.to_ascii_lowercase() as i32;
        vm.pushc(immediate);
        vm.wrchr();
        let output = String::from_utf8(output).expect("Not utf-8");
        assert_eq!(output, String::from("1"));
    }
    #[test]
    fn test_pushg() {
        let mut vm = NinjaVM::default();
        vm.sda = StaticDataArea::new(1, 0);
        let value = 2;
        vm.sda.memory[0] = value;
        vm.pushg(0);
        assert_eq!(vm.sda.memory[0], value);
    }
    #[test]
    fn test_popg() {
        let mut vm = NinjaVM::default();
        vm.sda = StaticDataArea::new(1, 0);
        let value = 2;
        vm.stack.push(value);
        vm.popg(0);
        assert_eq!(vm.sda.memory[0], value);
    }
    #[test]
    fn test_asf() {}
    #[test]
    fn test_rsf() {}
    #[test]
    fn test_pushl() {}
    #[test]
    fn test_popl() {}
}

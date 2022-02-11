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
        match self.reader.read_line(&mut input) {
            Ok(line) => line,
            Err(_) => fatal_error("Error: failed to read line"),
        };
        let immediate: Immediate = match input.trim().parse::<i32>() {
            Ok(line) => line,
            Err(_) => fatal_error("Error: input not an integer"),
        };
        self.stack.push(immediate)
    }
    pub fn wrint(&mut self) {
        match write!(self.writer, "{}", self.stack.pop()) {
            Ok(_) => {}
            Err(_) => fatal_error("Error: unable to write"),
        }
    }
    pub fn rdchr(&mut self) {
        let mut input = String::new();
        match self.reader.read_line(&mut input) {
            Ok(line) => line,
            Err(_) => fatal_error("Error: failed to read line"),
        };
        let immediate = match input.trim().chars().next() {
            Some(char) => char,
            None => fatal_error("Error: failed to read character"),
        } as Immediate;
        self.stack.push(immediate)
    }
    pub fn wrchr(&mut self) {
        let character = self.stack.pop() as u8 as char;
        match write!(self.writer, "{character}") {
            Ok(_) => {}
            Err(_) => fatal_error("Error: unable to write"),
        }
    }
    pub fn pushg(&mut self, immediate: Immediate) {
        self.stack.push(self.sda.memory[immediate as usize]);
    }
    pub fn popg(&mut self, immediate: Immediate) {
        self.sda.memory[immediate as usize] = self.stack.pop();
    }
    pub fn asf(&mut self, immediate: Immediate) {
        self.stack.push(self.stack.fp as Immediate);
        self.stack.fp = self.stack.sp;
        let mut stack_size = self.stack.memory.len();
        stack_size += immediate as usize;
        self.stack.memory.resize(stack_size, 0);
        self.stack.sp += immediate as usize;
    }
    pub fn rsf(&mut self) {
        let fp = self.stack.fp as usize;
        let sp = self.stack.sp as usize;
        let stack_size = self.stack.memory.len() - (sp - fp);
        self.stack.memory.resize(stack_size, 0);
        self.stack.sp = self.stack.fp;
        self.stack.fp = self.stack.pop() as usize;
    }
    pub fn pushl(&mut self, immediate: Immediate) {
        let fp = self.stack.fp as usize;
        let n = immediate as usize;
        self.stack.push(self.stack.memory[fp + n]);
    }
    pub fn popl(&mut self, immediate: Immediate) {
        let n = immediate as usize;
        let fp = self.stack.fp as usize;
        let sp = self.stack.sp as usize;
        self.stack.memory[fp + n] = self.stack.memory[sp - 1];
    }
    pub fn eq(&mut self) {
        println!("Called eq");
    }
    pub fn ne(&mut self) {
        println!("Called ne");
    }
    pub fn lt(&mut self) {
        println!("Called lt");
    }
    pub fn le(&mut self) {
        println!("Called le");
    }
    pub fn gt(&mut self) {
        println!("Called gt");
    }
    pub fn ge(&mut self) {
        println!("Called ge");
    }
    pub fn jmp(&mut self, immediate: Immediate) {
        println!("Called jmp with immediate {immediate}");
    }
    pub fn brf(&mut self, immediate: Immediate) {
        println!("Called brf with immediate {immediate}");
    }
    pub fn brt(&mut self, immediate: Immediate) {
        println!("Called brt with immediate {immediate}");
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
    fn test_asf() {
        let mut vm = NinjaVM::default();
        let immediate = 100 as Immediate;
        let sp = vm.stack.sp;
        vm.asf(immediate);
        assert_eq!(vm.stack.sp, (immediate + 1) as usize);
        assert_eq!(vm.stack.fp, sp + 1);
        for i in 0..immediate as usize {
            assert_eq!(vm.stack.memory[i], 0)
        }
    }
    #[test]
    fn test_rsf() {
        let mut vm = NinjaVM::default();
        let immediate = 100 as Immediate;
        vm.asf(immediate);
        vm.rsf();
        assert_eq!(vm.stack.sp, 0);
        assert_eq!(vm.stack.fp, 0);
        assert_eq!(vm.stack.memory.len(), 0);
    }
    #[test]
    fn test_pushl() {
        let mut vm = NinjaVM::default();
        let value_of_local_var = 10;
        let nth_local_var = 2;
        vm.asf(2);
        vm.pushc(value_of_local_var);
        vm.popl(nth_local_var);
        let sp = vm.stack.sp;
        vm.pushl(nth_local_var);
        assert_eq!(vm.stack.sp, sp + 1);
        assert_eq!(vm.stack.memory[vm.stack.sp - 1], value_of_local_var);
    }
    #[test]
    fn test_popl() {
        let mut vm = NinjaVM::default();
        let value_of_local_var = 10;
        let nth_local_var: usize = 0;
        vm.asf(2);
        vm.pushc(value_of_local_var);
        vm.popl(nth_local_var as i32);
        assert_eq!(vm.stack.memory[vm.stack.fp + nth_local_var], value_of_local_var);
    }
    #[test]
    fn test_eq() {}
    #[test]
    fn test_ne() {}
    #[test]
    fn test_lt() {}
    #[test]
    fn test_le() {}
    #[test]
    fn test_gt() {}
    #[test]
    fn test_ge() {}
    #[test]
    fn test_jmp() {}
    #[test]
    fn test_brf() {}
    #[test]
    fn test_brt() {}
}

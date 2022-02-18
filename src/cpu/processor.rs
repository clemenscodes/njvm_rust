use crate::{fatal_error, Immediate, NinjaVM};
use std::fmt::Debug;
use std::io::{BufRead, Write};

impl<R: BufRead + Debug, W: Write + Debug> NinjaVM<R, W> {
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
        let mut byte_buffer = [0];
        loop {
            if let Ok(()) = self.reader.read_exact(&mut byte_buffer) {
                match byte_buffer[0] {
                    b'-' => break,
                    b'0' => break,
                    b'1' => break,
                    b'2' => break,
                    b'3' => break,
                    b'4' => break,
                    b'5' => break,
                    b'6' => break,
                    b'7' => break,
                    b'8' => break,
                    b'9' => break,
                    b' ' => continue,
                    _ => fatal_error("Error: input is not an integer"),
                }
            } else {
                fatal_error("Error: could not read character")
            }
        }
        let mut buffer = vec![byte_buffer[0]];
        loop {
            if let Ok(()) = self.reader.read_exact(&mut byte_buffer) {
                match byte_buffer[0] {
                    b'0' => buffer.push(byte_buffer[0]),
                    b'1' => buffer.push(byte_buffer[0]),
                    b'2' => buffer.push(byte_buffer[0]),
                    b'3' => buffer.push(byte_buffer[0]),
                    b'4' => buffer.push(byte_buffer[0]),
                    b'5' => buffer.push(byte_buffer[0]),
                    b'6' => buffer.push(byte_buffer[0]),
                    b'7' => buffer.push(byte_buffer[0]),
                    b'8' => buffer.push(byte_buffer[0]),
                    b'9' => buffer.push(byte_buffer[0]),
                    _ => break,
                }
            } else {
                fatal_error("Error: could not read character")
            }
        }
        let immediate = match String::from_utf8(buffer).unwrap().parse() {
            Ok(immediate) => immediate,
            Err(_) => fatal_error("Error: integer is too big"),
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
        let mut byte_buffer = [0];
        match self.reader.read_exact(&mut byte_buffer) {
            Ok(_) => {}
            Err(_) => fatal_error("Error: could not read character"),
        };
        let immediate = byte_buffer[0] as Immediate;
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
        self.stack.push(self.sda.data[immediate as usize]);
    }
    pub fn popg(&mut self, immediate: Immediate) {
        self.sda.data[immediate as usize] = self.stack.pop()
    }
    pub fn asf(&mut self, immediate: Immediate) {
        self.stack.push(self.stack.fp as Immediate);
        self.stack.fp = self.stack.sp;
        let mut stack_size = self.stack.data.len();
        stack_size += immediate as usize;
        self.stack.data.resize(stack_size, 0);
        self.stack.sp += immediate as usize;
    }
    pub fn rsf(&mut self) {
        let fp = self.stack.fp;
        let sp = self.stack.sp;
        let stack_size = self.stack.data.len() - (sp - fp);
        self.stack.data.resize(stack_size, 0);
        self.stack.sp = self.stack.fp;
        self.stack.fp = self.stack.pop() as usize;
    }
    pub fn pushl(&mut self, immediate: Immediate) {
        let fp = self.stack.fp;
        let n = immediate as usize;
        self.stack.push(self.stack.data[fp + n]);
    }
    pub fn popl(&mut self, immediate: Immediate) {
        let n = immediate as usize;
        let fp = self.stack.fp;
        let sp = self.stack.sp;
        self.stack.data[fp + n] = self.stack.data[sp - 1];
    }
    pub fn eq(&mut self) {
        let b = self.stack.pop();
        let a = self.stack.pop();
        let result = if a == b { 1 } else { 0 };
        self.stack.push(result);
    }
    pub fn ne(&mut self) {
        let b = self.stack.pop();
        let a = self.stack.pop();
        let result = if a != b { 1 } else { 0 };
        self.stack.push(result);
    }
    pub fn lt(&mut self) {
        let b = self.stack.pop();
        let a = self.stack.pop();
        let result = if a < b { 1 } else { 0 };
        self.stack.push(result);
    }
    pub fn le(&mut self) {
        let b = self.stack.pop();
        let a = self.stack.pop();
        let result = if a <= b { 1 } else { 0 };
        self.stack.push(result);
    }
    pub fn gt(&mut self) {
        let b = self.stack.pop();
        let a = self.stack.pop();
        let result = if a > b { 1 } else { 0 };
        self.stack.push(result);
    }
    pub fn ge(&mut self) {
        let b = self.stack.pop();
        let a = self.stack.pop();
        let result = if a >= b { 1 } else { 0 };
        self.stack.push(result);
    }
    pub fn jmp(&mut self, immediate: Immediate) {
        self.ir.pc = immediate as usize;
    }
    pub fn brf(&mut self, immediate: Immediate) {
        if self.stack.pop() == 0 {
            self.ir.pc = immediate as usize;
        }
    }
    pub fn brt(&mut self, immediate: Immediate) {
        if self.stack.pop() == 1 {
            self.ir.pc = immediate as usize;
        }
    }
    pub fn call(&mut self, immediate: Immediate) {
        let ra = (self.ir.pc + 1) as Immediate;
        self.stack.push(ra);
        self.ir.pc = immediate as usize;
    }
    pub fn ret(&mut self) {
        self.ir.pc = self.stack.pop() as usize;
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
        assert_eq!(vm.stack.data[0], 2);
    }
    #[test]
    fn test_add() {
        let mut vm = NinjaVM::default();
        vm.pushc(-1);
        vm.pushc(2);
        vm.add();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.data[0], 1);
    }
    #[test]
    fn test_sub() {
        let mut vm = NinjaVM::default();
        vm.pushc(1);
        vm.pushc(2);
        vm.sub();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.data[0], -1);
    }
    #[test]
    fn test_mul() {
        let mut vm = NinjaVM::default();
        vm.pushc(-1);
        vm.pushc(-2);
        vm.mul();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.data[0], 2);
    }
    #[test]
    fn test_div() {
        let mut vm = NinjaVM::default();
        vm.pushc(-7);
        vm.pushc(-2);
        vm.div();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.data[0], 3);
        vm.pushc(-3);
        vm.div();
        assert_eq!(vm.stack.sp, 1);
        assert_eq!(vm.stack.data[0], -1);
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
        assert_eq!(vm.stack.data[0], -1);
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
    fn test_rdint_works() {
        let input = b" -123  456 -789   ";
        let mut vm = NinjaVM::new(&input[..], stdout());
        vm.rdint();
        assert_eq!(vm.stack.data[0], -123);
        vm.rdint();
        assert_eq!(vm.stack.data[1], 456);
        vm.rdint();
        assert_eq!(vm.stack.data[2], -789);
    }
    #[test]
    #[should_panic(expected = "Error: input is not an integer")]
    fn test_rdint_fails_not_an_integer() {
        std::panic::set_hook(Box::new(|_| {}));
        let input = b" 123 s  456  789   ";
        let mut vm = NinjaVM::new(&input[..], stdout());
        vm.rdint();
        assert_eq!(vm.stack.data[0], 123);
        vm.rdint();
    }
    #[test]
    #[should_panic(expected = "Error: integer is too big")]
    fn test_rdint_fails_too_big() {
        std::panic::set_hook(Box::new(|_| {}));
        let input = b" 12345 67892424234242   ";
        let mut vm = NinjaVM::new(&input[..], stdout());
        vm.rdint();
        assert_eq!(vm.stack.data[0], 12345);
        vm.rdint();
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
    fn test_rdchr_works() {
        let input = b"123 456";
        let mut vm = NinjaVM::new(&input[..], stdout());
        vm.rdchr();
        assert_eq!(vm.stack.data[0], '1' as Immediate);
        vm.rdchr();
        assert_eq!(vm.stack.data[1], '2' as Immediate);
        vm.rdchr();
        assert_eq!(vm.stack.data[2], '3' as Immediate);
        vm.rdchr();
        assert_eq!(vm.stack.data[3], ' ' as Immediate);
        vm.rdchr();
        assert_eq!(vm.stack.data[4], '4' as Immediate);
        vm.rdchr();
        assert_eq!(vm.stack.data[5], '5' as Immediate);
        vm.rdchr();
        assert_eq!(vm.stack.data[6], '6' as Immediate);
    }
    #[test]
    #[should_panic(expected = "Error: could not read character")]
    fn test_rdchr_fails() {
        std::panic::set_hook(Box::new(|_| {}));
        let input = b"";
        let mut vm = NinjaVM::new(&input[..], stdout());
        vm.rdchr();
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
        vm.sda.data[0] = value;
        vm.pushg(0);
        assert_eq!(vm.sda.data[0], value);
    }
    #[test]
    fn test_popg() {
        let mut vm = NinjaVM::default();
        vm.sda = StaticDataArea::new(1, 0);
        let value = 2;
        vm.stack.push(value);
        vm.popg(0);
        assert_eq!(vm.sda.data[0], value);
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
            assert_eq!(vm.stack.data[i], 0)
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
        assert_eq!(vm.stack.data.len(), 0);
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
        assert_eq!(vm.stack.data[vm.stack.sp - 1], value_of_local_var);
    }
    #[test]
    fn test_popl() {
        let mut vm = NinjaVM::default();
        let value_of_local_var = 10;
        let nth_local_var: usize = 0;
        vm.asf(2);
        vm.pushc(value_of_local_var);
        vm.popl(nth_local_var as i32);
        assert_eq!(vm.stack.data[vm.stack.fp + nth_local_var], value_of_local_var);
    }
    #[test]
    fn test_eq() {
        let mut vm = NinjaVM::default();
        vm.pushc(1);
        vm.pushc(2);
        vm.eq();
        assert_eq!(vm.stack.data[0], 0);
        vm.pushc(-1);
        vm.eq();
        assert_eq!(vm.stack.data[0], 0);
        vm.pushc(0);
        vm.eq();
        assert_eq!(vm.stack.data[0], 1);
    }
    #[test]
    fn test_ne() {
        let mut vm = NinjaVM::default();
        vm.pushc(1);
        vm.pushc(2);
        vm.ne();
        assert_eq!(vm.stack.data[0], 1);
        vm.pushc(-1);
        vm.ne();
        assert_eq!(vm.stack.data[0], 1);
        vm.pushc(1);
        vm.ne();
        assert_eq!(vm.stack.data[0], 0);
    }
    #[test]
    fn test_lt() {
        let mut vm = NinjaVM::default();
        vm.pushc(1);
        vm.pushc(2);
        vm.lt();
        assert_eq!(vm.stack.data[0], 1);
        vm.pushc(0);
        vm.lt();
        assert_eq!(vm.stack.data[0], 0);
        vm.pushc(1);
        vm.lt();
        assert_eq!(vm.stack.data[0], 1);
    }
    #[test]
    fn test_le() {
        let mut vm = NinjaVM::default();
        vm.pushc(1);
        vm.pushc(2);
        vm.le();
        assert_eq!(vm.stack.data[0], 1);
        vm.pushc(1);
        vm.le();
        assert_eq!(vm.stack.data[0], 1);
        vm.pushc(0);
        vm.le();
        assert_eq!(vm.stack.data[0], 0);
    }
    #[test]
    fn test_gt() {
        let mut vm = NinjaVM::default();
        vm.pushc(1);
        vm.pushc(2);
        vm.gt();
        assert_eq!(vm.stack.data[0], 0);
        vm.pushc(0);
        vm.gt();
        assert_eq!(vm.stack.data[0], 0);
        vm.pushc(-1);
        vm.gt();
        assert_eq!(vm.stack.data[0], 1);
    }
    #[test]
    fn test_ge() {
        let mut vm = NinjaVM::default();
        vm.pushc(1);
        vm.pushc(2);
        vm.ge();
        assert_eq!(vm.stack.data[0], 0);
        vm.pushc(0);
        vm.ge();
        assert_eq!(vm.stack.data[0], 1);
    }
    #[test]
    fn test_jmp() {
        let mut vm = NinjaVM::default();
        let immediate = 5;
        vm.jmp(immediate);
        assert_eq!(vm.ir.pc, immediate as usize);
    }
    #[test]
    fn test_brf() {
        let mut vm = NinjaVM::default();
        let immediate = 5;
        vm.pushc(1);
        vm.brf(immediate);
        assert_eq!(vm.ir.pc, 0);
        vm.pushc(0);
        vm.brf(immediate);
        assert_eq!(vm.ir.pc, immediate as usize);
    }
    #[test]
    fn test_brt() {
        let mut vm = NinjaVM::default();
        let immediate = 5;
        vm.pushc(0);
        vm.brt(immediate);
        assert_eq!(vm.ir.pc, 0);
        vm.pushc(1);
        vm.brt(immediate);
        assert_eq!(vm.ir.pc, immediate as usize);
    }
    #[test]
    fn test_call() {
        let mut vm = NinjaVM::default();
        vm.load("tests/data/a4/prog01.bin");
        vm.init();
        let immediate = 5;
        let ra = vm.ir.pc + 1;
        vm.call(immediate);
        assert_eq!(vm.ir.pc, immediate as usize);
        assert_eq!(vm.stack.sp, ra as usize);
        assert_eq!(vm.stack.data[vm.stack.sp - 1], ra as i32);
    }
    #[test]
    fn test_ret() {
        let mut vm = NinjaVM::default();
        vm.load("tests/data/a4/prog01.bin");
        vm.init();
        let immediate = 5;
        let ra = vm.ir.pc + 1;
        vm.pushc(2);
        vm.call(immediate);
        assert_eq!(vm.stack.data[1], ra as i32);
        assert_eq!(vm.ir.pc, immediate as usize);
        assert_ne!(vm.ir.pc, ra);
        vm.ret();
        assert_eq!(vm.stack.data[0], 2);
        assert_eq!(vm.stack.data.len(), 1);
        assert_eq!(vm.ir.pc, ra)
    }
}

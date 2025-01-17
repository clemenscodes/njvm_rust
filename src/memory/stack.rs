use std::{
    cell::RefCell,
    fmt::{Debug, Display, Formatter, Result},
    io::{BufRead, StderrLock, StdinLock, StdoutLock, Write},
    rc::Rc,
};

use crate::{cpu::immediate::Immediate, io::InputOutput};

pub type StackPointer = usize;
pub type FramePointer = usize;

#[derive(Eq, PartialEq, Clone)]
pub struct Stack<
    R: BufRead + Debug,
    W: Write + Debug,
    E: Write + Debug,
    T: Clone + Debug + Display,
> {
    pub io: Rc<RefCell<InputOutput<R, W, E>>>,
    pub sp: StackPointer,
    pub fp: FramePointer,
    pub data: Vec<T>,
}

impl Default
    for Stack<StdinLock<'_>, StdoutLock<'_>, StderrLock<'_>, Immediate>
{
    fn default() -> Self {
        let io = InputOutput::default();
        Self::new(Rc::new(RefCell::new(io)))
    }
}

impl<
        R: BufRead + Debug,
        W: Write + Debug,
        E: Write + Debug,
        T: Clone + Debug + Display,
    > Stack<R, W, E, T>
{
    pub fn new(io: Rc<RefCell<InputOutput<R, W, E>>>) -> Self {
        Stack {
            io,
            sp: 0,
            fp: 0,
            data: vec![],
        }
    }
    pub fn push(&mut self, immediate: T) {
        self.data.push(immediate);
        self.sp += 1;
    }
    pub fn pop(&mut self) -> T {
        if self.sp == 0 || self.data.is_empty() {
            self.io
                .borrow()
                .fatal_error("Stack underflow: popped from empty stack");
        }
        self.sp -= 1;
        match self.data.pop() {
            Some(immediate) => immediate,
            None => self
                .io
                .borrow()
                .fatal_error("Stack underflow: popped from empty stack"),
        }
    }
    pub fn print(&self) {
        println!("{self:#?}");
    }
}

impl<
        R: BufRead + Debug,
        W: Write + Debug,
        E: Write + Debug,
        T: Clone + Debug + Display,
    > Debug for Stack<R, W, E, T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let sp = self.sp;
        let fp = self.fp;
        for slot in (0..=self.data.len()).rev() {
            if sp == 0 && fp == 0 {
                write!(f, "sp, fp --->\t{slot:04}:\txxxx")?;
            } else if sp == fp {
                writeln!(f, "sp, fp --->\t{slot:04}:\t{}", self.data[slot])?;
            }
            if slot != sp && slot != fp {
                writeln!(f, "\t\t{slot:04}:\t{}", self.data[slot])?;
            }
            if slot == sp && slot != fp {
                writeln!(f, "sp \t --->\t{sp:04}:\txxxx")?;
            }
            if slot == fp && slot != sp && fp == 0 {
                write!(f, "fp \t --->\t{fp:04}:\t{}", self.data[fp])?;
            }
            if slot == fp && slot != sp && fp != 0 {
                writeln!(f, "fp \t --->\t{fp:04}:\t{}", self.data[fp])?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let stack = Stack::<
            StdinLock<'_>,
            StdoutLock<'_>,
            StderrLock<'_>,
            Immediate,
        >::default();
        assert_eq!(stack.sp, 0);
        assert_eq!(stack.data.len(), 0);
    }

    #[test]
    fn test_push() {
        let mut stack = Stack::<
            StdinLock<'_>,
            StdoutLock<'_>,
            StderrLock<'_>,
            Immediate,
        >::default();
        stack.push(1);
        assert_eq!(stack.sp, 1);
        assert_eq!(stack.data[0], 1);
        stack.push(5);
        assert_eq!(stack.sp, 2);
        assert_eq!(stack.data[1], 5);
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::<
            StdinLock<'_>,
            StdoutLock<'_>,
            StderrLock<'_>,
            Immediate,
        >::default();
        stack.push(1);
        assert_eq!(stack.sp, 1);
        assert_eq!(stack.data[0], 1);
        assert_eq!(stack.pop(), 1);
        assert_eq!(stack.sp, 0);
        assert_eq!(stack.data.len(), 0);
    }

    #[test]
    #[should_panic(expected = "Stack underflow: popped from empty stack")]
    fn test_stack_underflow() {
        std::panic::set_hook(Box::new(|_| {}));
        let mut stack = Stack::<
            StdinLock<'_>,
            StdoutLock<'_>,
            StderrLock<'_>,
            Immediate,
        >::default();
        stack.pop();
    }
}

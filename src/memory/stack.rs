use crate::{fatal_error, Immediate};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Stack<T> {
    pub sp: u32,
    pub memory: Vec<T>,
}

impl Default for Stack<Immediate>
where
    Immediate: std::fmt::Debug + std::fmt::Display,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Stack<T>
where
    T: std::fmt::Debug + std::fmt::Display,
{
    pub fn new() -> Self {
        Stack { sp: 0, memory: vec![] }
    }
    pub fn push(&mut self, immediate: T) {
        self.memory.push(immediate);
        self.sp += 1;
    }
    pub fn pop(&mut self) -> T {
        if self.sp == 0 && self.memory.is_empty() {
            fatal_error("Stack underflow: popped from empty stack");
        }
        self.sp -= 1;
        match self.memory.pop() {
            Some(immediate) => immediate,
            None => fatal_error("Stack underflow: popped from empty stack"),
        }
    }
    pub fn print(&self) {
        println!("{self:#?}");
    }
}

#[cfg(test)]
mod tests {
    use crate::{Immediate, Stack};
    #[test]
    fn test_stack() {
        let stack = Stack::<Immediate>::default();
        assert_eq!(stack.sp, 0);
        assert_eq!(stack.memory.len(), 0);
    }
    #[test]
    fn test_push() {
        let mut stack = Stack::<Immediate>::default();
        stack.push(1);
        assert_eq!(stack.sp, 1);
        assert_eq!(stack.memory[0], 1);
        stack.push(5);
        assert_eq!(stack.sp, 2);
        assert_eq!(stack.memory[1], 5);
    }
    #[test]
    fn test_pop() {
        let mut stack = Stack::<Immediate>::default();
        stack.push(1);
        assert_eq!(stack.sp, 1);
        assert_eq!(stack.memory[0], 1);
        assert_eq!(stack.pop(), 1);
        assert_eq!(stack.sp, 0);
        assert_eq!(stack.memory.len(), 0);
    }
    #[test]
    #[should_panic(expected = "Stack underflow: popped from empty stack")]
    fn test_stack_underflow() {
        std::panic::set_hook(Box::new(|_| {}));
        let mut stack = Stack::<Immediate>::default();
        stack.pop();
    }
}

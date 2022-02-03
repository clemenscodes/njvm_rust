use crate::{Immediate, MAXITEMS};

#[derive(Debug, Eq, PartialEq)]
pub struct Stack {
    pub sp: u32,
    pub memory: Vec<Immediate>,
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
        Stack { sp: 0, memory: vec![] }
    }
    pub fn push(&mut self, immediate: Immediate) {
        if self.sp > MAXITEMS {
            panic!("Stack overflow: Stack is full, not more than {MAXITEMS} allowed");
        }
        self.memory.push(immediate);
        self.sp += 1;
    }
    pub fn pop(&mut self) -> Immediate {
        if self.sp == 0 && self.memory.is_empty() {
            panic!("Stack underflow: popped from empty stack");
        }
        self.sp -= 1;
        self.memory.pop().expect("Stack underflow: popped from empty stack")
    }
    pub fn print(&self) {
        println!("{self:#?}");
    }
}

#[cfg(test)]
mod tests {
    use crate::Stack;
    #[test]
    fn test_stack() {
        let stack = Stack::default();
        assert_eq!(stack.sp, 0);
        assert_eq!(stack.memory.len(), 0);
    }
    #[test]
    fn test_push() {
        let mut stack = Stack::default();
        stack.push(1);
        assert_eq!(stack.sp, 1);
        assert_eq!(stack.memory[0], 1);
        stack.push(5);
        assert_eq!(stack.sp, 2);
        assert_eq!(stack.memory[1], 5);
    }
    #[test]
    #[should_panic]
    fn test_stack_overflow() {
        std::panic::set_hook(Box::new(|_| {}));
        let mut stack = Stack::default();
        for i in 0..=10001 {
            stack.push(i);
        }
    }
    #[test]
    fn test_pop() {
        let mut stack = Stack::default();
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
        let mut stack = Stack::default();
        stack.pop();
    }
}

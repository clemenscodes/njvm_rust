use crate::{Immediate, MAXITEMS};

#[derive(Debug)]
pub struct Stack {
    sp: u32,
    memory: [Immediate; MAXITEMS as usize],
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            sp: 0,
            memory: [0; MAXITEMS as usize],
        }
    }
    pub fn push(&mut self, immediate: Immediate) {
        if self.sp > MAXITEMS as u32 {
            panic!("Stack overflow: Stack is full, not more than {MAXITEMS} allowed");
        }
        self.memory[self.sp as usize] = immediate;
        self.sp += 1;
    }
    pub fn pop(&mut self) -> Immediate {
        if self.sp == 0 && self.memory[self.sp as usize] == 0 {
            panic!("Stack underflow: popped from empty stack");
        }
        self.sp -= 1;
        let tmp = self.memory[self.sp as usize];
        self.memory[self.sp as usize] = 0;
        tmp
    }
    pub fn print(&self) {
        println!("{self:#?}");
    }
}

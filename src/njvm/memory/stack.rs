use std::process::exit;

pub const MAXITEMS: u8 = 100;

#[derive(Debug)]
pub struct Stack {
    sp: u32,
    memory: [u32; MAXITEMS as usize],
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            sp: 0,
            memory: [0; MAXITEMS as usize],
        }
    }
    pub fn push(&mut self, immediate: u32) {
        if self.sp > MAXITEMS as u32 {
            println!("Stack overflow: Stack is full, not more than {MAXITEMS} allowed");
            exit(1)
        }
        self.memory[self.sp as usize] = immediate;
        self.sp = self.sp + 1;
    }
    pub fn pop(&mut self) -> u32 {
        if self.sp == 0 && self.memory[self.sp as usize] == 0 {
            println!("Stack underflow: popped from empty stack");
            exit(1)
        }
        self.sp = self.sp - 1;
        let tmp = self.memory[self.sp as usize];
        self.memory[self.sp as usize] = 0;
        tmp
    }
    pub fn print_stack(&self) {
        for stack_item in (0..self.sp).rev() {
            if stack_item == self.sp {
                println!("{self:#?}")
            } else {
                println!("{self:#?}")
            }
        }
    }
}

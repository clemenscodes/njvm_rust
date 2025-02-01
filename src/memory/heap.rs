use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    io::{BufRead, StderrLock, StdinLock, StdoutLock, Write},
    rc::Rc,
};

use crate::{cpu::immediate::Immediate, io::InputOutput};

pub type HeapPointer = usize;
pub type FramePointer = usize;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Heap<
    R: BufRead + Debug,
    W: Write + Debug,
    E: Write + Debug,
    T: Clone + Debug + Display,
> {
    pub io: Rc<RefCell<InputOutput<R, W, E>>>,
    pub data: Vec<T>,
}

impl Default
    for Heap<StdinLock<'_>, StdoutLock<'_>, StderrLock<'_>, Immediate>
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
    > Heap<R, W, E, T>
{
    pub fn new(io: Rc<RefCell<InputOutput<R, W, E>>>) -> Self {
        Heap { io, data: vec![] }
    }
}

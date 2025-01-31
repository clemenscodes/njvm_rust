use std::{
    cell::RefCell,
    fmt::{Debug, Display, Formatter, Result},
    io::{BufRead, StderrLock, StdinLock, StdoutLock, Write},
    rc::Rc,
};

use crate::{cpu::immediate::Immediate, io::InputOutput};

#[derive(Eq, PartialEq, Clone)]
pub struct StaticDataArea<
    R: BufRead + Debug,
    W: Write + Debug,
    E: Write + Debug,
    T,
> {
    pub io: Rc<RefCell<InputOutput<R, W, E>>>,
    pub data: Vec<T>,
}

impl Default
    for StaticDataArea<StdinLock<'_>, StdoutLock<'_>, StderrLock<'_>, Immediate>
{
    fn default() -> Self {
        let io = InputOutput::default();
        Self::new(Rc::new(RefCell::new(io)), 0, 0)
    }
}

impl<
        R: BufRead + Debug,
        W: Write + Debug,
        E: Write + Debug,
        T: Clone + Debug + Display,
    > StaticDataArea<R, W, E, T>
{
    pub fn new(
        io: Rc<RefCell<InputOutput<R, W, E>>>,
        size: usize,
        value: T,
    ) -> Self {
        let mut data = vec![];
        data.resize(size, value);
        StaticDataArea { io, data }
    }

    pub fn print(&self) {
        let output = format!("{self:#?}");
        self.io.borrow().write_stdout(&output);
    }
}

impl<
        R: BufRead + Debug,
        W: Write + Debug,
        E: Write + Debug,
        T: Debug + Display,
    > Debug for StaticDataArea<R, W, E, T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for data in 0..self.data.len() {
            if data == (self.data.len() - 1) {
                write!(f, "data[{data:04}]:\t{}", self.data[data])?;
            } else {
                writeln!(f, "data[{data:04}]:\t{}", self.data[data])?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sda() {
        let sda = StaticDataArea::default();
        assert_eq!(sda.data.len(), 0);
    }
}

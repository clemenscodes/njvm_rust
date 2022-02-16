use crate::Immediate;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Eq, PartialEq, Clone)]
pub struct StaticDataArea<T> {
    pub data: Vec<T>,
}

impl Default for StaticDataArea<Immediate> {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl<T: Clone + Debug + Display> StaticDataArea<T> {
    pub fn new(size: usize, value: T) -> Self {
        let mut data = vec![];
        data.resize(size, value);
        StaticDataArea { data }
    }
    pub fn print(&self) {
        println!("{self:#?}");
    }
}

impl<T: Debug + Display> Debug for StaticDataArea<T> {
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

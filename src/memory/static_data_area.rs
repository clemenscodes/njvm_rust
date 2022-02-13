use crate::Immediate;
use std::fmt::{Debug, Display};

#[derive(Debug, Eq, PartialEq, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sda() {
        let sda = StaticDataArea::default();
        assert_eq!(sda.data.len(), 0);
    }
}

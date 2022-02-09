use crate::Immediate;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StaticDataArea<T> {
    pub memory: Vec<T>,
}

impl Default for StaticDataArea<Immediate>
where
    Immediate: std::fmt::Debug + std::fmt::Display,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> StaticDataArea<T>
where
    T: std::fmt::Debug + std::fmt::Display,
{
    pub fn new() -> Self {
        StaticDataArea { memory: Vec::new() }
    }
    pub fn push(&mut self, immediate: T) {
        self.memory.push(immediate);
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
        assert_eq!(sda.memory.len(), 0);
    }
    #[test]
    fn test_push() {
        let mut sda = StaticDataArea::default();
        sda.push(1);
        assert_eq!(sda.memory[0], 1);
    }
}

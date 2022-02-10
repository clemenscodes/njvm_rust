use crate::fatal_error;

pub fn split_file_metadata(file: &mut Vec<u8>) -> Vec<u8> {
    if file.len() < 16 {
        fatal_error("Error: code file is corrupted")
    }
    file.split_off(16)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split_file_metadata_works() {
        let mut f = Vec::new();
        f.resize(16, 0);
        let len = f.len();
        let instructions = split_file_metadata(&mut f);
        assert_eq!(f.len(), 16);
        assert_eq!(instructions.len(), len - f.len());
    }
    #[test]
    #[should_panic(expected = "Error: code file is corrupted")]
    fn test_split_file_metadata_fails() {
        std::panic::set_hook(Box::new(|_| {}));
        let mut f = Vec::new();
        split_file_metadata(&mut f);
    }
}

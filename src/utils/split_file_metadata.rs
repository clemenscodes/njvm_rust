use crate::fatal_error;

pub fn split_file_metadata(file: &mut Vec<u8>) -> Vec<u8> {
    let content = file.split_off(16);
    if file.len() < 16 {
        fatal_error("Error: code file is corrupted")
    }
    content
}

#[cfg(test)]
mod tests {
    use crate::{read_file, split_file_metadata};
    #[test]
    fn test_split_file_metadata() {
        let path = "../tests/data/a2/prog1.bin";
        let mut f = read_file(&path);
        let len = f.len();
        let instructions = split_file_metadata(&mut f);
        assert_eq!(f.len(), 16);
        assert_eq!(instructions.len(), len - f.len());
    }
}

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
    use super::*;
    #[test]
    fn test_split_file_metadata() {
        let mut f = Vec::new();
        split_file_metadata(&mut f);
    }
}

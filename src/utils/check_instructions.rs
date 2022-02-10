use crate::fatal_error;

pub fn check_instructions(file: &mut Vec<u8>) -> usize {
    match file
        .chunks_mut(4)
        .nth(2)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
    {
        Some(count) => match count.try_into() {
            Ok(count) => count,
            Err(_) => fatal_error("Failed to parse to usize from u32"),
        },
        None => fatal_error("Failed to read instruction count"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{check_instructions, read_file};
    #[test]
    fn test_check_instructions() {
        let path = "tests/data/a2/prog1.bin";
        let mut f = read_file(path);
        check_instructions(&mut f);
    }
}

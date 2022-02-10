use crate::fatal_error;

pub fn check_instructions(file: &[u8]) -> usize {
    match file
        .chunks(4)
        .nth(2)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
    {
        Some(count) => match count.try_into() {
            Ok(count) => count,
            Err(_) => fatal_error("Error: failed to parse to usize from u32"),
        },
        None => fatal_error("Error: failed to read instruction count"),
    }
}

#[cfg(test)]
mod tests {
    use crate::check_instructions;
    #[test]
    fn test_check_instruction_works() {
        let mut f = Vec::new();
        f.resize(12, 0);
        let instruction_count = check_instructions(&f);
        assert_eq!(instruction_count, 0);
        f[8] = 1;
        let instruction_count = check_instructions(&f);
        assert_eq!(instruction_count, 1);
    }
    #[test]
    #[should_panic(expected = "Error: failed to read instruction count")]
    fn test_check_instruction_fails() {
        std::panic::set_hook(Box::new(|_| {}));
        let mut f = Vec::new();
        check_instructions(&mut f);
    }
}

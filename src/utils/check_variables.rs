use crate::fatal_error;

pub fn check_variables(file: &[u8]) -> usize {
    match file
        .chunks(4)
        .nth(3)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
    {
        Some(count) => match count.try_into() {
            Ok(count) => count,
            Err(_) => fatal_error("Error: failed to parse to usize from u32"),
        },
        None => fatal_error("Error: failed to read global variable count"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_variables_works() {
        let mut f = Vec::new();
        f.resize(16, 0);
        let instruction_count = check_variables(&f);
        assert_eq!(instruction_count, 0);
        f[12] = 1;
        let instruction_count = check_variables(&f);
        assert_eq!(instruction_count, 1);
    }
    #[test]
    #[should_panic(expected = "Error: failed to read global variable count")]
    fn test_check_variables_fails() {
        std::panic::set_hook(Box::new(|_| {}));
        let mut f = Vec::new();
        check_variables(&mut f);
    }
}

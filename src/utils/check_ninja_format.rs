pub fn check_ninja_format(file: &[u8], arg: &str) {
    let ninja_binary_format = &[78, 74, 66, 70];
    if !file.starts_with(ninja_binary_format) {
        eprintln!("Error: file '{arg}' is not a Ninja binary");
        #[cfg(not(test))]
        std::process::exit(1);
        #[cfg(test)]
        panic!("Error: file '{arg}' is not a Ninja binary");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_ninja_format_works() {
        let f = vec![78, 74, 66, 70];
        check_ninja_format(&f, "test_file");
    }
    #[test]
    #[should_panic(expected = "Error: file 'test_file' is not a Ninja binary")]
    fn test_check_ninja_format_fails() {
        std::panic::set_hook(Box::new(|_| {}));
        let f = vec![];
        check_ninja_format(&f, "test_file");
    }
}

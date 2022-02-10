use crate::fatal_error;

pub fn read_file(arg: &str) -> Vec<u8> {
    if arg.is_empty() {
        fatal_error("Error: no code file specified");
    }
    match std::fs::read(arg) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: cannot open code file '{arg}'");
            #[cfg(not(test))]
            std::process::exit(1);
            #[cfg(test)]
            panic!("Error: cannot open code file '{arg}'");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_file_works() {
        let path = "Cargo.toml";
        let f = read_file(path);
        assert_eq!(f[18] as char, 'n');
        assert_eq!(f[19] as char, 'j');
        assert_eq!(f[20] as char, 'v');
        assert_eq!(f[21] as char, 'm');
    }
    #[test]
    #[should_panic(expected = "Error: cannot open code file '/'")]
    fn test_read_file_fails() {
        std::panic::set_hook(Box::new(|_| {}));
        read_file("/");
    }
}

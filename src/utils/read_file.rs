use crate::utils::fatal_error::fatal_error;

pub fn read_file(arg: &str) -> Vec<u8> {
    if arg.trim().is_empty() {
        fatal_error("Error: no code file specified");
    }

    std::fs::read(arg).unwrap_or_else(|_| {
        let error_message = format!("Error: cannot open code file '{arg}'");
        #[cfg(not(test))]
        {
            eprintln!("{}", error_message);
            std::process::exit(1);
        }
        #[cfg(test)]
        {
            panic!("{}", error_message);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_read_file_works() {
        let test_file = "test_file.txt";
        let mut file = File::create(test_file).expect("Failed to create test file");
        write!(file, "This is a test file").expect("Failed to write to test file");
        let content = read_file(test_file);
        assert_eq!(content, b"This is a test file");
        fs::remove_file(test_file).expect("Failed to remove test file");
    }

    #[test]
    #[should_panic(expected = "Error: cannot open code file '/'")]
    fn test_read_file_invalid_path() {
        read_file("/");
    }

    #[test]
    #[should_panic(expected = "Error: no code file specified")]
    fn test_read_file_empty_argument() {
        read_file("");
    }

    #[test]
    #[should_panic(expected = "Error: cannot open code file 'nonexistent.txt'")]
    fn test_read_file_nonexistent_file() {
        read_file("nonexistent.txt");
    }
}

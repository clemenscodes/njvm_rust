pub fn read_file(arg: &str) -> Vec<u8> {
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
    fn test_read_file() {
        let path = "tests/data/a2/prog1.bin";
        read_file(path);
    }
    #[test]
    #[should_panic(expected = "Error: cannot open code file 'tests/data/a2/prog1.404'")]
    fn test_read_file_fails() {
        let path = "tests/data/a2/prog1.404";
        read_file(path);
    }
}

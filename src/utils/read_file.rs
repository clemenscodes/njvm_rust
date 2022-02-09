use std::fs;

pub fn read_file(arg: &str) -> Vec<u8> {
    match fs::read(arg) {
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
        read_file("");
    }
}

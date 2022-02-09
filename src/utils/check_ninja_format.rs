pub fn check_ninja_format(file: &mut Vec<u8>, arg: &str) {
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
    use crate::{check_ninja_format, read_file};
    #[test]
    fn test_check_ninja_format() {
        let path = "../tests/data/a2/prog1.bin";
        let mut f = read_file(path);
        check_ninja_format(&mut f, path);
    }
    #[test]
    #[should_panic(expected = "Error: file '../tests/data/a2/prog1.asm' is not a Ninja binary")]
    fn test_check_ninja_format_fails() {
        let path = "../tests/data/a2/prog1.asm";
        let mut f = read_file(path);
        check_ninja_format(&mut f, path);
    }
}

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
    use super::*;
    #[test]
    fn test_check_ninja_format() {
        let mut f = Vec::new();
        check_ninja_format(&mut f, "");
    }
}

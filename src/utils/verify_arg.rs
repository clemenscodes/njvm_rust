use crate::utils::unknown_arg::unknown_arg;

pub fn verify_arg(arg: &str) {
    if arg.starts_with('-') {
        unknown_arg(arg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_arg() {
        verify_arg("should be interpreted as file path");
    }
    #[test]
    #[should_panic(expected = "unknown command line argument '--not-a-file', try './njvm --help'")]
    fn test_verify_arg_fails() {
        std::panic::set_hook(Box::new(|_| {}));
        verify_arg("--not-a-file");
    }
}

pub fn unknown_arg(arg: &str) {
    eprintln!("unknown command line argument '{arg}', try './njvm --help'");
    #[cfg(not(test))]
    std::process::exit(1);
    #[cfg(test)]
    panic!("unknown command line argument '{arg}', try './njvm --help'");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(
        expected = "unknown command line argument '--unknown-arg', try './njvm --help'"
    )]
    fn test_unknown_arg() {
        std::panic::set_hook(Box::new(|_| {}));
        unknown_arg("--unknown-arg")
    }
}

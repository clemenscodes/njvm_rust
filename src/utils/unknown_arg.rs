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
    fn test_unknown_arg() {
        unknown_arg("")
    }
}

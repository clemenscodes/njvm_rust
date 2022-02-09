pub fn fatal_error(error: &str) -> ! {
    eprintln!("{error}");
    #[cfg(not(test))]
    std::process::exit(1);
    #[cfg(test)]
    panic!("{error}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic(expected = "should panic")]
    fn test_fatal_error() {
        fatal_error("should panic")
    }
}
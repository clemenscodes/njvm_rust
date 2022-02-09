use crate::unknown_arg;

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
        verify_arg("");
    }
}

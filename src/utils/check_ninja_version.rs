use crate::fatal_error;

pub const VERSION: u8 = 3;

pub fn check_ninja_version(file: &[u8]) {
    let version = match file
        .chunks(4)
        .nth(1)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
    {
        Some(version) => version,
        None => fatal_error("Failed to read version"),
    };
    if VERSION != version as u8 {
        fatal_error("Error: code file does not have correct Ninja version")
    }
}

#[cfg(test)]
mod tests {
    use crate::{check_ninja_version, VERSION};
    #[test]
    fn test_check_ninja_version_works() {
        let mut f = Vec::new();
        f.resize(8, 0);
        f[4] = VERSION;
        check_ninja_version(&f);
    }
    #[test]
    #[should_panic(expected = "Error: code file does not have correct Ninja version")]
    fn test_check_ninja_version_fails() {
        std::panic::set_hook(Box::new(|_| {}));
        let mut f = Vec::new();
        f.resize(8, 0);
        check_ninja_version(&f);
    }
}

use crate::fatal_error;

pub const VERSION: u32 = 2;

pub fn check_ninja_version(file: &mut Vec<u8>) {
    let version = match file
        .chunks_mut(4)
        .nth(1)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
    {
        Some(version) => version,
        None => fatal_error("Failed to read version"),
    };
    if VERSION != version {
        fatal_error("Error: code file is not a Ninja binary")
    }
}

#[cfg(test)]
mod tests {
    use crate::{check_ninja_version, read_file};
    #[test]
    fn test_check_ninja_version() {
        let path = "../tests/data/a2/prog1.bin";
        let mut f = read_file(path);
        check_ninja_version(&mut f);
    }
    #[test]
    #[should_panic(expected = "Error: code file is not a Ninja binary")]
    fn test_check_ninja_version_fails() {
        let path = "../tests/data/a2/prog1.asm";
        let mut f = read_file(path);
        check_ninja_version(&mut f);
    }
}

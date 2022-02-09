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
        fatal_error("Error: invalid version")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_ninja_version() {
        let mut f = Vec::new();
        check_ninja_version(&mut f);
    }
}

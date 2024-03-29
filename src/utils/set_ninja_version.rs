use crate::{fatal_error, VERSION};

pub fn set_ninja_version(file: &mut [u8]) {
    let version = match file.chunks_mut(4).nth(1).map(|c| {
        c[0] = VERSION;
        u32::from_le_bytes([c[0], c[1], c[2], c[3]])
    }) {
        Some(version) => version,
        None => fatal_error("Failed to read version"),
    };
    if VERSION != version as u8 {
        fatal_error("Error: code file does not have correct Ninja version")
    }
}

#[cfg(test)]
mod tests {
    use crate::{set_ninja_version, VERSION};
    #[test]
    fn test_set_ninja_version_works() {
        let mut f = Vec::new();
        f.resize(8, 0);
        set_ninja_version(&mut f);
        assert_eq!(f[4], VERSION);
    }
}

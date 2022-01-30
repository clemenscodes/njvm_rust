use anyhow::Result;
use assert_cmd::{crate_name, Command};
use predicates::str::contains;

#[test]
pub fn njvm_help_works() -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(contains("usage: ./njvm [option] [option] ..."));
    Ok(())
}

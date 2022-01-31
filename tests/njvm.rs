use anyhow::Result;
use assert_cmd::{crate_name, Command};
use predicates::str::contains;

#[test]
pub fn help_works() -> Result<()> {
    let output = "usage: ./njvm [option] [option] ...";
    test_output_success("--help", &output)
}

#[test]
pub fn version_works() -> Result<()> {
    let output = "Ninja Virtual Machine version 1 (compiled Sep 23 2015, 10:36:52)";
    test_output_success("--version", &output)
}

#[test]
pub fn unknown_command_fails() -> Result<()> {
    let output = "unknown command line argument";
    test_output_failure("--unknown-arg", &output)
}

fn test_output_success(arg: &str, output: &str) -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg(arg);
    cmd.assert().success().stdout(contains(output));
    Ok(())
}

fn test_output_failure(arg: &str, output: &str) -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg(arg);
    cmd.assert().failure().stderr(contains(output));
    Ok(())
}

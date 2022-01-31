use anyhow::Result;
use assert_cmd::{crate_name, Command};
use difference::Changeset;
use predicates::str::contains;

#[test]
pub fn njvm_works() -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.assert().success();
    Ok(())
}

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

#[test]
pub fn prog1_works() -> Result<()> {
    let output = read_file("tests/data/prog1.out")?;
    test_output("--prog1", &output)
}

#[test]
pub fn prog2_works() -> Result<()> {
    let output = read_file("tests/data/prog2.out")?;
    test_stdin_output("--prog2", "10", &output)
}

#[test]
pub fn prog3_works() -> Result<()> {
    let output = read_file("tests/data/prog3.out")?;
    test_stdin_output("--prog3", "1", &output)
}

fn read_file(filename: &str) -> Result<String> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content)
}

fn test_stdin_output(arg: &str, stdin: &str, output: &str) -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    let stdout = cmd.arg(arg).write_stdin(stdin).output()?.stdout;
    let stdout = String::from_utf8(stdout)?;
    let output = String::from(output);
    let changeset = Changeset::new(&output, &stdout, "");
    println!("{changeset}");
    assert_eq!(output, stdout);
    Ok(())
}

fn test_output(arg: &str, output: &str) -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    let stdout = cmd.arg(arg).output()?.stdout;
    let stdout = String::from_utf8(stdout)?;
    let output = String::from(output);
    let changeset = Changeset::new(&output, &stdout, "");
    println!("{changeset}");
    assert_eq!(output, stdout);
    Ok(())
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
    cmd.assert().failure().code(1).stderr(contains(output));
    Ok(())
}

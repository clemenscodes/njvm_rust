use anyhow::Result;
use assert_cmd::{crate_name, Command};
use difference::Changeset;
use predicates::str::contains;

#[test]
pub fn njvm_works() -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    let output = format!("Error: no code file specified\n");
    let stdout = cmd.assert().failure();
    let stderr = stdout.get_output().stderr.clone();
    let err = String::from_utf8(stderr)?;
    assert_eq!(output, err);
    Ok(())
}

#[test]
pub fn help_works() -> Result<()> {
    let output = "usage: ./njvm [options] <code file>";
    test_output_success("--help", &output)
}

#[test]
pub fn version_works() -> Result<()> {
    let output = "Ninja Virtual Machine version 2 (compiled Sep 23 2015, 10:36:52)";
    test_output_success("--version", &output)
}

#[test]
pub fn unknown_command_fails() -> Result<()> {
    let output = "unknown command line argument";
    test_output_failure("--unknown-arg", &output)
}

#[test]
pub fn prog1_works() -> Result<()> {
    let output = read_file("tests/data/a2/prog1.out")?;
    test_output("tests/data/a2/prog1.bin", &output)
}

#[test]
pub fn prog2_works() -> Result<()> {
    let output = read_file("tests/data/a2/prog2.out")?;
    test_output("tests/data/a2/prog2.bin", &output)
}

#[test]
pub fn prog1_1_works() -> Result<()> {
    let output = read_file("tests/data/a1/prog1.out")?;
    test_output("tests/data/a1/prog1.bin", &output)
}

#[test]
pub fn prog2_1_works() -> Result<()> {
    let output = read_file("tests/data/a1/prog2.out")?;
    test_stdin_output("tests/data/a1/prog2.bin", "10", &output)
}

#[test]
pub fn prog3_1_works() -> Result<()> {
    let output = read_file("tests/data/a1/prog3.out")?;
    test_stdin_output("tests/data/a1/prog3.bin", "10", &output)
}

fn read_file(filename: &str) -> Result<String> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content)
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

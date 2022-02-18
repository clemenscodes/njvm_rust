use anyhow::Result;
use assert_cmd::{crate_name, Command};
use difference::Changeset;
use predicates::str::contains;
// use std::process::Command as Cmd;

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
    let output = format!("Ninja Virtual Machine version 4 (compiled Sep 23 2015, 10:36:52)");
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
    cmd.assert().failure().code(1).stderr(contains(output));
    Ok(())
}

#[test]
pub fn prog1_works() -> Result<()> {
    let base = "tests/data/a4/prog01";
    let file = format!("{base}.bin");
    let output = format!("{base}.out");
    let output = read_file(&output)?;
    test_output(&file, &output)
}

#[test]
pub fn prog2_works() -> Result<()> {
    let base = "tests/data/a4/prog02";
    let file = format!("{base}.bin");
    let output = format!("{base}.out");
    let output = read_file(&output)?;
    test_output(&file, &output)
}
// #[test]
// pub fn prog3_works() -> Result<()> {
//     let base = "tests/data/a4/prog03";
//     let file = format!("{base}.bin");
//     let output = format!("{base}.out");
//     let output = read_file(&output)?;
//     test_output(&file, &output)
// }
// #[test]
// pub fn prog4_works() -> Result<()> {
//     let base = "tests/data/a4/prog04";
//     let file = format!("{base}.bin");
//     let output = format!("{base}.out");
//     let output = read_file(&output)?;
//     test_output(&file, &output)
// }
// #[test]
// pub fn prog10_works() -> Result<()> {
//     let base = "tests/data/a4/prog10";
//     let file = format!("{base}.bin");
//     let output = format!("{base}.out");
//     let output = read_file(&output)?;
//     test_stdin_output(&file, "8 12", &output)
// }
// #[test]
// pub fn prog11_works() -> Result<()> {
//     let base = "tests/data/a4/prog11";
//     let file = format!("{base}.bin");
//     let output = format!("{base}.out");
//     let output = read_file(&output)?;
//     test_stdin_output(&file, "5", &output)
// }
// #[test]
// pub fn prog12_works() -> Result<()> {
//     let base = "tests/data/a4/prog12";
//     let file = format!("{base}.bin");
//     let output = format!("{base}.out");
//     let output = read_file(&output)?;
//     test_stdin_output(&file, "5", &output)
// }

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

// fn test_stdin_output(arg: &str, stdin: &str, output: &str) -> Result<()> {
//     let custom_arg = format!("echo {} | target/x86_64-unknown-linux-gnu/release/njvm {}", stdin, arg);
//     println!("{custom_arg}");
//     let mut cmd = Cmd::new("sh");
//     let stdout = cmd
//         .arg("-c")
//         .arg(custom_arg)
//         .stdout(std::process::Stdio::piped())
//         .output()?
//         .stdout;
//     let stdout = String::from_utf8(stdout)?;
//     let output = String::from(output);
//     let changeset = Changeset::new(&output, &stdout, "");
//     println!("{changeset}");
//     assert_eq!(output, stdout);
//     Ok(())
// }

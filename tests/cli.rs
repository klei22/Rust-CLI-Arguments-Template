use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;


type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_with_f_and_no_file() -> TestResult {
    Command::cargo_bin("cli_template")?.arg("f").assert().failure().stderr(predicate::str::contains("USAGE"));
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("cli_template")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn file_to_stdout() -> TestResult {
    run(&["-f","just_a_test.txt"], "tests/expected/print_text.txt")
}

#[test]
fn file_to_all_caps() -> TestResult {
    run(&["-A", "-f","just_a_test.txt"], "tests/expected/print_text_as_all_caps.txt")
}

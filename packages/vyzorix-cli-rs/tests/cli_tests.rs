use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("vyzorix").unwrap();
    let assert = cmd.arg("--help").assert();
    assert
        .success()
        .stdout(predicate::str::contains("Vyzorix Workspace Management CLI"));
}

#[test]
fn test_doctor_command() {
    let mut cmd = Command::cargo_bin("vyzorix").unwrap();
    let assert = cmd.arg("doctor").assert();
    assert
        .success()
        .stdout(predicate::str::contains("Running Vyzorix Infrastructure Diagnostics"));
}

#[test]
fn test_dev_flag_override() {
    let mut cmd = Command::cargo_bin("vyzorix").unwrap();
    let assert = cmd.arg("--dev").arg("doctor").assert();
    assert
        .success()
        .stdout(predicate::str::contains("Running in Development Mode"));
}

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn set_project_title() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("nextup")?;
    cmd.arg("set").arg("b").arg("test text");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test text"));

    Ok(())
}

#[test]
fn set_project_nextup() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("nextup")?;
    cmd.arg("b").arg("test nextup");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test text"));

    Ok(())
}

#[test]
fn reset_project() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("nextup")?;
    cmd.arg("reset");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("reset all projects"));

    Ok(())
}

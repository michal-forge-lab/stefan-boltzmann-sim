use assert_cmd::Command;
use predicates::prelude::*;


#[test]
fn fails_without_args() {
    let mut cmd = Command::cargo_bin("radiation-sim").unwrap();
    cmd.assert().failure();
}


#[test]
fn runs_with_valid_args() {
    let mut cmd = Command::cargo_bin("radiation-sim").unwrap();
    cmd.args(["-T", "500", "-A", "2.0", "-t", "10"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("P [W]"))
        .stdout(predicate::str::contains("E [J]"));
}
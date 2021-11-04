use assert_cmd::Command;

const COMMAND: &str = env!("CARGO_PKG_NAME");

#[test]
fn output_does_not_exit() {
    let mut cmd = Command::cargo_bin(COMMAND).unwrap();

    cmd.args(&["--output", "./path/does/not/exists"])
        .assert()
        .code(1)
        .failure();
}

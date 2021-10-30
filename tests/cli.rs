use assert_cmd::Command;

const COMMAND: &str = env!("CARGO_PKG_NAME");

#[test]
fn output_does_not_exit() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(COMMAND).unwrap();

    cmd.args(&["./path/does/not/exists"])
        .assert()
        .code(1)
        .failure();

    Ok(())
}

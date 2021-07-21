use assert_cmd::Command;

#[test]
fn test_check() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("issue");
    cmd.arg("check");
    cmd.args(["--manifest-path", "../tests/Cargo.toml"]);

    let output = cmd.assert();
    output.success().stderr("");
}

#[test]
fn test_list() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("issue");
    cmd.arg("list");
    cmd.args(["--manifest-path", "../tests/Cargo.toml"]);

    let output = cmd.assert();
    output.success().stderr("");
}

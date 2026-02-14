use assert_cmd::Command;

#[test]

fn runs() {
    let mut cmd = Command::cargo_bin("hello-world").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}
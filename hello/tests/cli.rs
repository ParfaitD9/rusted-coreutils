use assert_cmd::Command;

#[test]
/// Test hello program exit correctly & show `Hello, world!`
fn works() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
/// Test true program exit correctly with status code 0
fn true_is_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
/// Test false program fails with status code 1
fn false_is_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

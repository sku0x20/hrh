use std::env;
use std::process::Command;
use std::process::Stdio;

#[test]
fn e2e() {
    let binary_path = env::args().nth(2).unwrap();

    let output = Command::new(binary_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
}

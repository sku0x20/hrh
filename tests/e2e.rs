use std::env;
use std::process::Command;
use std::process::Stdio;

#[test]
fn e2e() {
    let binary_path = env::args().last().unwrap();

    let output = Command::new(binary_path)
        .args([
            "-f",
            "tests/resources/vm-agent.yaml",
            "--helm-path",
            "fake_helm.sh",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    println!("{:#?}", output);
}

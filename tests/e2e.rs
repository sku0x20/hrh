use std::fs;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use std::process::Stdio;

const OUTPUT_FILE: &str = "fake_helm.out";
const EXE_PATH: &str = "target/debug/hrh";

#[test]
fn release() {
    let binary_path = EXE_PATH;

    fs::remove_file(OUTPUT_FILE).unwrap_or_else(|_| {
        eprintln!("failed to remove fake_helm.out");
    });

    let output = Command::new(binary_path)
        .args([
            "-f",
            "tests/resources/vm-agent.yaml",
            "--helm-path",
            "./fake_helm.sh",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let mut fake_helm_out =
        File::open(OUTPUT_FILE).expect(&format!("failed to open {}", OUTPUT_FILE));
    let mut output = String::new();
    fake_helm_out
        .read_to_string(&mut output)
        .expect(&format!("failed to open {}", OUTPUT_FILE));

    assert_eq!(
        output,
        "upgrade --atomic --install pod-collector vm/victoria-metrics-agent --namespace observability --values tests/resources/pod-collector.yaml\n"
    );
}

#[test]
fn diff() {
    let binary_path = EXE_PATH;

    fs::remove_file(OUTPUT_FILE).unwrap_or_else(|_| {
        eprintln!("failed to remove fake_helm.out");
    });

    let output = Command::new(binary_path)
        .args([
            "--diff",
            "-f",
            "tests/resources/vm-agent.yaml",
            "--helm-path",
            "./fake_helm.sh",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let mut fake_helm_out =
        File::open(OUTPUT_FILE).expect(&format!("failed to open {}", OUTPUT_FILE));
    let mut output = String::new();
    fake_helm_out
        .read_to_string(&mut output)
        .expect(&format!("failed to open {}", OUTPUT_FILE));

    assert_eq!(
        output,
        "diff upgrade --namespace observability --allow-unreleased pod-collector vm/victoria-metrics-agent --values tests/resources/pod-collector.yaml\n"
    );
}

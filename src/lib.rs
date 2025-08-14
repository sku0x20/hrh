use std::process::{Command, Stdio};

pub mod config;

pub fn run(config: config::Config) {
    println!("{:?}", config);
    execute_helm(config.helm_path);
}

fn execute_helm(helm: String) {
    Command::new(helm)
        .args(["--something", "tests/resources.out"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute helm");
}

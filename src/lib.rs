use crate::declaration::Declaration;
use std::fs::File;
use std::process::{Command, Stdio};

pub mod config;
mod declaration;

pub fn run(config: config::Config) {
    println!("{:?}", config);
    let file = File::open(config.file).expect("file not found");
    let declaration: Declaration = serde_yaml::from_reader(file).expect("failed to parse yaml");
    println!("{:?}", declaration);
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

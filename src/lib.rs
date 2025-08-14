use crate::config::Config;
use crate::declaration::Declaration;
use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};

pub mod config;
mod declaration;

pub fn run(config: Config) {
    println!("{:?}", config);
    let file = File::open(&config.file).expect("file not found");
    let mut declaration: Declaration = serde_yaml::from_reader(file).expect("failed to parse yaml");
    declaration = transform(declaration, &config);
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

fn transform(
    declaration: Declaration,
    config: &Config,
) -> Declaration {
    let mut tranformed = declaration.clone();
    transform_file_path(&mut tranformed, &config.file);
    return tranformed;
}

fn transform_file_path(
    declaration: &mut Declaration,
    config_file: &str,
) {
    let file_path = Path::new(config_file);
    let values_file_path = Path::new(&declaration.values_file);
    if values_file_path.is_relative() {
        let parent_path = file_path.parent().expect("failed to get parent path");
        declaration.values_file =
            String::from(parent_path.join(values_file_path).to_str().unwrap());
    }
}

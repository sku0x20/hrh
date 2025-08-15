use crate::config::Config;
use crate::declaration::Declaration;
use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};

pub mod config;
mod declaration;
mod logger;

pub fn run(config: Config) {
    debug!("{:?}", config);
    let file = File::open(&config.file).expect("file not found");
    let mut declaration: Declaration = serde_yaml::from_reader(file).expect("failed to parse yaml");
    declaration = transform(declaration, &config);
    debug!("{:?}", declaration);
    let helm_args = helm_args(declaration);
    execute_helm(config.helm_path, helm_args);
}

fn execute_helm(
    helm: String,
    args: Vec<String>,
) {
    Command::new(helm)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute helm");
}

fn helm_args(declaration: Declaration) -> Vec<String> {
    return vec![
        String::from("upgrade"),
        String::from("--install"),
        declaration.release_name,
        format!("{}/{}", declaration.repo, declaration.chart_name),
        String::from("--namespace"),
        declaration.namespace,
        String::from("--values"),
        declaration.values_file,
    ];
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

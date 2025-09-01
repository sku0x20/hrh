use crate::config::Config;
use crate::release::Release;
use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};

pub mod config;
pub mod logger;
mod release;

pub fn run(config: Config) {
    debug!("{:?}", config);
    let file = File::open(&config.file).expect("file not found");
    let mut release: Release = serde_yaml::from_reader(file).expect("failed to parse yaml");
    release = transform(release, &config);
    debug!("{:?}", release);
    let helm_args = helm_args(&config, release);
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

fn helm_args(
    config: &Config,
    release: Release,
) -> Vec<String> {
    if config.is_diff {
        vec![
            String::from("diff"),
            String::from("upgrade"),
            String::from("--namespace"),
            release.namespace,
            String::from("--allow-unreleased"),
            release.release_name,
            format!("{}/{}", release.repo, release.chart_name),
            String::from("--values"),
            release.values_file,
            String::from("--version"),
            release.version,
        ]
    } else {
        vec![
            String::from("upgrade"),
            String::from("--atomic"),
            String::from("--install"),
            release.release_name,
            format!("{}/{}", release.repo, release.chart_name),
            String::from("--namespace"),
            release.namespace,
            String::from("--values"),
            release.values_file,
            String::from("--version"),
            release.version,
        ]
    }
}

fn transform(
    release: Release,
    config: &Config,
) -> Release {
    let mut tranformed = release.clone();
    transform_file_path(&mut tranformed, &config.file);
    return tranformed;
}

fn transform_file_path(
    release: &mut Release,
    config_file: &str,
) {
    let file_path = Path::new(config_file);
    let values_file_path = Path::new(&release.values_file);
    if values_file_path.is_relative() {
        let parent_path = file_path.parent().expect("failed to get parent path");
        release.values_file = String::from(parent_path.join(values_file_path).to_str().unwrap());
    }
}

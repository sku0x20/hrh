use hrh::Config;
use std::vec::IntoIter;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut config = Config::new();
    parse_args(args.into_iter(), &mut config)
}

fn parse_args(
    mut into_iter: IntoIter<String>,
    config: &mut Config,
) {
    into_iter.next();
    while let Some(flag) = into_iter.next() {
        parse_arg(&flag, &mut into_iter, config)
    }
}

fn parse_arg(
    flag: &str,
    into_iter: &mut IntoIter<String>,
    config: &mut Config,
) {
    match flag {
        "--file" | "-f" => {
            config.file = into_iter.next().unwrap();
        }
        "--helm-path" => {
            config.helm_path = into_iter.next().unwrap();
        }
        _ => {
            println!("Unknown flag: {}", flag);
        }
    }
}

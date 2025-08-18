use hrh::config::Config;
use hrh::logger::init_logger;
use hrh::run;
use std::vec::IntoIter;

fn main() {
    init_logger();

    let args: Vec<String> = std::env::args().collect();
    let mut config = Config::new();
    parse_args(args.into_iter(), &mut config);
    run(config);
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
        "--diff" => config.is_diff = true,
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

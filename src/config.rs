#[derive(Debug)]
pub struct Config {
    pub file: String,
    pub helm_path: String,
    pub is_diff: bool,
}

impl Config {
    pub fn new() -> Self {
        Config {
            file: String::new(),
            helm_path: String::from("helm"),
            is_diff: true,
        }
    }
}

pub struct Config {
    pub file: String,
    pub helm_path: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            file: String::new(),
            helm_path: String::from("helm"),
        }
    }
}

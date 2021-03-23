// Implements a way to save the configuration of a project with syncro
pub struct Config {
    files: Vec<String>,
}

impl Config {
    pub fn new() -> Config {
        Config { files: vec![] }
    }
}

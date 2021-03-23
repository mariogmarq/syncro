use std::io::BufRead;
use std::path::PathBuf;

// name of the config file
const CONFIG_FILE_NAME: &str = ".syncro";

// Implements a way to save the configuration of a project with syncro
pub struct Config {
    files: Vec<String>,
}

impl Config {
    pub fn new() -> Config {
        Config { files: vec![] }
    }

    //TODO: Implement a way to write, read and find configuration files

    // Reads the configuration file in the given path
    pub fn read(&mut self, folder: &std::path::PathBuf) {
        let mut file = folder.clone();
        file.push(CONFIG_FILE_NAME);
        if !file.exists() || file.is_dir() {
            //TODO: Implement error handling, maybe with nice reporting :)
        }

        //Since file exists, we read it
        let file = std::fs::File::open(file.as_path()).unwrap();
        let mut reader = std::io::BufReader::new(file);
        let mut line = String::new();

        loop {
            match reader.read_line(&mut line) {
                Ok(bytes) => {
                    if bytes == 0 {
                        break;
                    }
                    self.files.push(line.clone());
                }

                Err(_) => {
                    //Error handling
                }
            }
        }
    }
}

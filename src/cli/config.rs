use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fs, io::BufRead};

/// name of the config file
const CONFIG_FILE_NAME: &str = ".syncro";

/// Implements a way to save the configuration of a project with syncro
pub struct Config {
    files: Vec<String>,
    path: Option<std::path::PathBuf>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            files: vec![],
            path: None,
        }
    }

    /// Reads the configuration file in the given path, must include the name
    /// Calling this function will store the path into the config struct
    pub fn read_from_path(&mut self, file: &PathBuf) {
        if !file.exists() || file.is_dir() {
            //TODO: Implement error handling, maybe with nice reporting :)
        }

        //Since file exists, we read it
        self.path = Some(fs::canonicalize(file.clone()).expect("No valid config file"));
        let file = std::fs::File::open(file.as_path()).unwrap();
        let mut reader = std::io::BufReader::new(file);
        let mut line = String::new();

        loop {
            match reader.read_line(&mut line) {
                Ok(bytes) => {
                    if bytes == 0 {
                        //EOF
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

    /// Creates the configuration file in the given path, doesn't have to include the config file
    /// name(optional)
    pub fn create_in_path(path: &PathBuf) -> std::result::Result<std::fs::File, std::io::Error> {
        // Comprobation of folder
        let mut true_path = PathBuf::new();
        if !path.is_dir() {
            // if is not a dir, we go to the parent dir
            true_path = path.parent().unwrap().to_path_buf();
        } else {
            true_path = path.clone();
        }

        // The file path
        true_path.push(CONFIG_FILE_NAME);

        //Create file, open it and return it
        std::fs::File::create(true_path.clone())?;
        let file = std::fs::File::open(true_path)?;
        Ok(file)
    }

    /// Finds the configuration file either in the local folder on an upper one
    pub fn find_folder() -> Option<PathBuf> {
        // Folder where we are searching
        let mut folder = PathBuf::new();
        folder.push(".");

        loop {
            //If current dir is root, we break
            if folder.as_path() == Path::new("/") {
                break;
            }

            //Read the actual dir
            let entries = match std::fs::read_dir(folder.clone()) {
                Ok(dir) => dir,
                Err(_) => break,
            };

            //Look if any entry have the name of the file
            let mut entries =
                entries.filter(|x| x.as_ref().unwrap().file_name() == CONFIG_FILE_NAME);

            match entries.next() {
                //Found the file, return his path
                Some(e) => {
                    return Some(e.unwrap().path());
                }
                None => folder.push(".."), // If not, go one directory up
            }
        }

        //No file have been found
        None
    }

    /// Write the configuration into the file
    /// Option requiered if path field is None
    pub fn write(&self, mut config_file: Option<std::fs::File>) -> std::io::Result<()> {
        //Change open options for write

        match &self.path {
            Some(path) => {
                config_file = Some(
                    OpenOptions::new()
                        .write(true)
                        .open(path.as_path())
                        .expect("Couldn't open the file"),
                )
            }
            None => {}
        }

        let mut writer = std::io::BufWriter::new(config_file.expect("No file specified"));

        for value in &self.files {
            match writer.write(value.as_bytes()) {
                Err(e) => eprintln!("ERROR: {}", e),
                Ok(_) => {}
            }
            writer.write(b"\n").expect("ERROR writing");
        }

        Ok(())
    }

    /// Loads the configuration
    pub fn load(&mut self) {
        //TODO: error handling
        self.files.clear();

        match Config::find_folder() {
            Some(file) => self.read_from_path(&file),
            None => {}
        }
    }

    /// Adds a path into the configuration
    // if it exists does nothing
    pub fn add(&mut self, path: &PathBuf) {
        let path = std::fs::canonicalize(path).expect("Couldn't resolve path");
        let path = path.to_str().expect("A valid path");
        let path = String::from_str(path).expect("A valid path");
        match self.files.binary_search(&path) {
            Ok(_) => {}
            Err(index) => self.files.insert(index, path),
        }
    }

    /// Delete a file from the configuration
    pub fn delete(&mut self, path: &PathBuf) {
        let index = self
            .files
            .iter()
            .position(|x| Path::new(&x) == path.as_path());

        match index {
            None => {}
            Some(i) => {
                self.files.remove(i);
            }
        };
    }
}

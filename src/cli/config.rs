use std::fs::DirEntry;
use std::io::BufRead;

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
    pub fn read_from_path(&mut self, path: &std::path::PathBuf) {
        let mut file = path.clone();
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

    // Creates the configuration file in the given path
    fn create_in_path(path: &std::path::PathBuf) -> std::io::Result<()> {
        // Comprobation of folder
        let mut true_path = std::path::PathBuf::new();
        if !path.is_dir() {
            // if is not a dir, we go to the parent dir
            true_path = path.parent().unwrap().to_path_buf();
        } else {
            true_path = path.clone();
        }

        // The file path
        true_path.push(CONFIG_FILE_NAME);

        //Create file and write
        std::fs::File::create(true_path)?;

        //Return
        Ok(())
    }

    // Finds the configuration file either in the local folder on an upper one
    // //TODO: testing
    fn find_folder() -> Option<std::path::PathBuf> {
        // Folder where we are searching
        let mut folder = std::path::PathBuf::new();
        folder.push(".");

        loop {
            //If current dir is root, we break
            if folder.as_path() == std::path::Path::new("/") {
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
}

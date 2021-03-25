use super::super::cli::config::Config;
use std::io::{Error, ErrorKind};

pub fn init() -> std::result::Result<std::fs::File, std::io::Error> {
    match Config::find_folder() {
        Some(_) => {
            let err = Error::new(ErrorKind::Other, "File already exists");
            std::result::Result::<std::fs::File, Error>::Err(err)
        }
        None => Config::create_in_path(&std::path::Path::new(".").to_path_buf()),
    }
}

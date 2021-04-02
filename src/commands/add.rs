use std::path::PathBuf;

use super::super::cli::config::Config;

pub fn add(files: &Vec<PathBuf>, cfg: &mut Config) -> std::result::Result<(), std::io::Error> {
    cfg.load();
    for file in files {
        cfg.add(file);
    }
    cfg.write(None)
}

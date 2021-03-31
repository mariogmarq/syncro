use std::path::PathBuf;

use super::super::cli::config::Config;

pub fn add(files: &Vec<PathBuf>, cfg: &mut Config) {
    cfg.load();
    for file in files {
        cfg.add(file);
    }
}

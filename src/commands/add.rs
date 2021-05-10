use std::path::PathBuf;

use super::super::cli::config::Config;

pub fn add(files: &[PathBuf], cfg: &mut Config) {
    cfg.load();
    for file in files {
        cfg.add(file);
    }
    cfg.write(None);
}

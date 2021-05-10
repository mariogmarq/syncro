use super::super::cli::config::Config;
use std::path::PathBuf;

pub fn delete(files: &[PathBuf], cfg: &mut Config) {
    cfg.load();
    for file in files {
        cfg.delete(file);
    }

    cfg.write(None);
}

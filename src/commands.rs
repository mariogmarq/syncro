mod add;
mod delete;
mod init;
mod update;

use std::path::PathBuf;

pub fn init() {
    match init::init() {
        Ok(_) => {}
        Err(e) => println!("ERROR: {}", e),
    }
}

pub fn add(files: &[PathBuf], cfg: &mut super::cli::config::Config) {
    add::add(files, cfg);
}

pub fn delete(files: &[PathBuf], cfg: &mut super::cli::config::Config) {
    delete::delete(files, cfg);
}

pub fn update(cfg: &mut super::cli::config::Config) {
    update::update(cfg);
}

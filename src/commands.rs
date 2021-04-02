mod add;
mod init;

pub fn init() {
    match init::init() {
        Ok(_) => {}
        Err(e) => println!("ERROR: {}", e),
    }
}

pub fn add(files: &Vec<std::path::PathBuf>, cfg: &mut super::cli::config::Config) {
    match add::add(files, cfg) {
        Ok(_) => {}
        Err(e) => println!("ERROR: {}", e),
    }
}

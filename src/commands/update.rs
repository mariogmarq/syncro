use super::super::cli::config::Config;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::result::Result;

pub fn update(cfg: &mut Config) {
    cfg.load();
    let files: Vec<PathBuf> = cfg
        .files()
        .iter()
        .map(|x| Path::new(x).to_path_buf())
        .collect();

    for file in files {
        if !file.exists() {
            println!("Skipping {} since it doesn't exists", file.display());
            continue;
        }

        let working_dir = cfg
            .working_folder()
            .expect("Config file didn't load properly");

        match file.is_dir() {
            true => match handle_dir(file, working_dir.clone()) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                }
            },
            false => match handle_file(file, working_dir.clone()) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                }
            },
        }
    }
}

/// It must be a dir, no comprobation of it is done here!!!
// is what to be copied, and working_dir where
fn handle_dir(path: PathBuf, working_dir: PathBuf) -> Result<(), io::Error> {
    // Create a dir with the name in the working dir if it exists
    let mut dirname = working_dir;
    dirname.push(path.file_name().unwrap());
    if let Err(e) = fs::metadata(dirname.clone()) {
        if e.kind() == ErrorKind::NotFound {
            fs::create_dir(dirname.as_path())?;
        }
    }

    for entry in fs::read_dir(path).expect("Error reading folder") {
        let entry = entry?;
        match entry.path().is_dir() {
            true => {
                handle_dir(entry.path(), dirname.clone())?;
            }
            false => {
                handle_file(entry.path(), dirname.clone())?;
            }
        }
    }

    Ok(())
}

// path is the path of the file to be copied, working dir is where to be copied
fn handle_file(path: PathBuf, mut working_dir: PathBuf) -> Result<(), io::Error> {
    working_dir.push(path.file_name().expect("Error copying file"));
    //Working dir is now the name of the future file
    println!("Copying {} into {}", path.display(), working_dir.display());
    fs::copy(path, working_dir)?;

    Ok(())
}

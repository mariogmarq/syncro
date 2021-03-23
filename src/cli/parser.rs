// Implements a command line parser through structopt

use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq)]
pub enum Command {
    /// Starts a syncro copy in the current folder
    Init,
    /// Add a file to the current copy, you must be in a started copy
    Add {
        #[structopt(parse(from_os_str))]
        files: Vec<std::path::PathBuf>,
    },
    /// Syncronizes the copy
    Update,
    /// Deletes a file from the current copy
    Delete {
        #[structopt(parse(from_os_str))]
        files: Vec<std::path::PathBuf>,
    },
    /// Sustitutes/Creates files based on the copy's ones
    Restore,
}

/// Syncro
///
/// Allows you to syncronize files between diferent folders
// Gets the arguments from the command line
#[derive(Debug, StructOpt)]
#[structopt(name = "syncro", about = "Syncronize files between folders")]
pub struct Syncro {
    /// Syncro subcommand
    // The subcommand in syncro
    #[structopt(subcommand)]
    sub: Command,
}

impl Syncro {
    pub fn subcommand(&self) -> Command {
        return self.sub;
    }
}

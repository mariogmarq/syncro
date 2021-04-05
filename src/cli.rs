// Implements a wrapper for both the argument parser and the configuration and create the commands
pub mod config;
mod parser;
use super::commands;
use structopt::StructOpt;

/// Wrapper for config and parser
pub struct Cli {
    /// Parser
    parser: parser::Syncro,
    /// Config
    config: config::Config,
}

impl Cli {
    //Creates a new CLI from command line
    pub fn new() -> Cli {
        let parser = parser::Syncro::from_args();
        let config = config::Config::new();
        Cli { parser, config }
    }

    /// Only function of Syncro, match each command with his own handler
    pub fn command(&mut self) {
        match &self.parser.subcommand() {
            parser::Command::Add { files } => commands::add(files, &mut self.config),
            parser::Command::Init => commands::init(),
            parser::Command::Update => commands::update(&mut self.config),
            parser::Command::Delete { files } => commands::delete(files, &mut self.config),
        }
    }
}

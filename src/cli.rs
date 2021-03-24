// Implements a wrapper for both the argument parser and the configuration and create the commands

mod config;
mod parser;
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
    pub fn command(&self) {
        match &self.parser.subcommand() {
            parser::Command::Add { files } => {}
            parser::Command::Init => {}
            parser::Command::Update => {}
            parser::Command::Delete { files } => {}
            parser::Command::Restore => {}
        }
    }
}

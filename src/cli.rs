mod parser;
use structopt::StructOpt;

// Wrapper for config and parser
pub struct Cli {
    //Parser
    parser: parser::Syncro,
    //Config
    //TODO: Config
}

impl Cli {
    //Creates a new CLI from command line
    pub fn new() -> Cli {
        let parser = parser::Syncro::from_args();
        Cli { parser }
    }

    //Commands
    pub fn command(&self) {
        self.parser.command();
    }
}

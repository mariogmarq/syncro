mod cli;
mod commands;

fn main() {
    let syncro = cli::Cli::new();
    syncro.command();
}

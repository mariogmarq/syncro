mod cli;
mod commands;

fn main() {
    let mut syncro = cli::Cli::new();
    syncro.command();
}

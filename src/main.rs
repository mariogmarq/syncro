mod cli;

fn main() {
    let syncro = cli::Cli::new();
    syncro.command();
}

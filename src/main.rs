use dirs;
use clap::Parser;

mod cli;

fn main() {
    let cli = cli::Cli::parse();
    println!("{}", cli.get_store_path().unwrap().display());
}

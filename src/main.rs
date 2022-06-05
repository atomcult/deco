use std::fs;
use std::path::PathBuf;

use clap::Parser;
use anyhow::Result;

mod cli;

enum PathMode {
    Add(PathBuf),
    Rm(PathBuf),
}

// TODO: Add ability to write to file
//       Weigh benefits of human-readable vs. binary format
//       My money is on human-readable, though, AKA serde_json, probably
// TODO: Expand dirstack at run-time to enumerate all the directories/files
// TODO: Find a way to remove operations that negate one another,
//       e.g. +DIR_A +DIR_B -DIR_A = +DIR_B
struct State {
    dirstack: Vec<PathMode>,
}

// TODO: Make sure to read in the state at some point!
fn main() {
    let cli = cli::Cli::parse();

    let store_path = cli.get_store_path()
        .expect("Unable to retrieve store path");
    init_dir(&store_path)
        .expect("Unable to initialize store path");

    let data_path = cli.get_data_path()
        .expect("Unable to retrieve data path");
    init_dir(&data_path)
        .expect("Unable to initialize data path");

    // TODO: At least for more complicated commands (i.e. select),
    //       make the command into its own struct, instead of just
    //       and enum struct. Might need methods for them anyhow.
    if let Some(cmd) = cli.command {
        match cmd {
            cli::Cmd::Add { files, all } => {
                println!("ADD: {:?}", files);
            },
            cli::Cmd::Rm { files, all } => {
                println!("RM:  {:?}", files);
            },
            
            // FIXME: Here just for testing, Cmd enum should be matched
            //        exaustively
            _ => {},
        }
    }
}

fn init_dir(path: &PathBuf) -> Result<()> {
    if ! path.exists() { fs::create_dir_all(path)?; }
    Ok(())
}

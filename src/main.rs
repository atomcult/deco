use std::fs;
use std::path::PathBuf;

// FIXME: debug only
use dotenv;

use clap::Parser;
use anyhow::{Result, Context};

mod cli;
mod state;

fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let cli = cli::Cli::parse();

    let store_path = cli.get_store_path()
        .context("Unable to retrieve store path")?;
    init_dir(&store_path)
        .context("Unable to initialize store path")?;

    let data_path = cli.get_data_path()
        .context("Unable to retrieve data path")?;
    init_dir(&data_path)
        .context("Unable to initialize data path")?;

    // Load the state from disk
    let mut state = state::State::load(&data_path)?;

    // NOTE: Should there be an init command to explicitly initialize
    //       directories and state file?
    // TODO: At least for more complicated commands (e.g. select),
    //       make the command into its own struct, instead of just
    //       and enum struct. Might need methods for them anyhow.
    // TODO: Make either `select` or `ls` command default
    // TODO: Only add path rules to dirstack if the path exists?
    // TODO: Allow regex or other pattern matching to the dirstack?
    if let Some(cmd) = cli.command {
        match cmd {
            cli::Cmd::Add { paths, all } => {
                // Add each path to the state (if any)
                for path in paths { state.add(path); }

                // Add all paths to the state if requested
                if all {
                    state.clear();
                    state.add(PathBuf::from("."));
                }

                // Save the state to disk
                state.save().context("Unable to save state")?;
            },

            cli::Cmd::Rm { paths, all } => {
                // Remove each path from the state (if any)
                for path in paths { state.rm(path); }

                // Clear the state if requested
                if all { state.clear(); }

                // Save the state to disk
                state.save().context("Unable to save state")?;
            },

            cli::Cmd::Ls => { ls(&state); },

            cli::Cmd::Select => {
                // TODO: Select a random file using the rules in the dirstack
            },
        }
    } else { ls(&state); }

    Ok(())
}

fn init_dir(path: &PathBuf) -> Result<()> {
    if ! path.exists() { fs::create_dir_all(path)?; }
    Ok(())
}

fn ls(state: &state::State) {
    for item in &state.dirstack {
        match item {
            state::PathMode::Add(path) => {
                println!("+{}", path.display());
            },
            state::PathMode::Rm(path) => {
                println!("-{}", path.display());
            },
        }
    }
}

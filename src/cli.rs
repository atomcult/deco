use std::path::PathBuf;

use dirs;
use clap::{Parser, Subcommand};

#[derive(Parser)]
// #[clap(name = "deco")]
#[clap(author, version, about)]
#[clap(long_about = None)]
#[clap(help_heading = "GLOBAL OPTIONS")]
pub struct Cli {
    #[clap(long, parse(from_os_str), value_name = "PATH", global = true)]
    #[clap(env = "DECO_STORE_PATH", hide_env_values = true)]
    #[clap(help = "Wallpaper storage path")]
    store_path: Option<PathBuf>,

    #[clap(long, parse(from_os_str), value_name = "PATH", global = true)]
    #[clap(env = "DECO_DATA_PATH", hide_env_values = true)]
    #[clap(help = "Deco data path")]
    data_path: Option<PathBuf>,

    #[clap(subcommand)]
    pub command: Option<Cmd>,
}

impl Cli {
    pub fn get_store_path(&self) -> Option<PathBuf> {
        self.store_path.clone().or_else(|| {
            dirs::home_dir().map(|p| p.join(".deco"))
        })
    }

    pub fn get_data_path(&self) -> Option<PathBuf> {
        self.data_path.clone().or_else(|| {
            dirs::data_local_dir().map(|p| p.join("deco"))
        })
    }
}

#[derive(Subcommand)]
pub enum Cmd {
    Add {
        #[clap(conflicts_with = "all")]
        files: Vec<PathBuf>,

        #[clap(long, short = 'A')]
        #[clap(conflicts_with = "files")]
        all: bool,
    },
    Rm {
        #[clap(conflicts_with = "all")]
        files: Vec<PathBuf>,

        #[clap(long, short = 'A')]
        #[clap(conflicts_with = "files")]
        all: bool,
    },

    // Select {

    // },

    // Save {

    // },

    // Delete {

    // },

    // Import {

    // },
}

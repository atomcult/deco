use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

use serde::{Serialize, Deserialize};
use serde_json as json;
use anyhow::Result;

const STATE_FNAME: &'static str = "state.json";

#[derive(Serialize, Deserialize)]
pub enum PathMode {
    Add(PathBuf),
    Rm(PathBuf),
}

// TODO: Expand dirstack at run-time to enumerate all the directories/files
// TODO: Find a way to remove operations that negate one another,
//       e.g. +DIR_A +DIR_B -DIR_A = +DIR_B
// TODO: State object should be versioned to update older versions as it is changed
pub struct State {
    fname: PathBuf,
    pub dirstack: Vec<PathMode>,
}

impl State {
    pub fn add(&mut self, path: PathBuf) {
        self.dirstack.push(PathMode::Add(path));
    }

    pub fn rm(&mut self, path: PathBuf) {
        self.dirstack.push(PathMode::Rm(path));
    }

    pub fn clear(&mut self) {
         self.dirstack = Vec::new();
    }

    pub fn save(&self) -> Result<()> {
        let data = json::to_vec_pretty(&self.dirstack)?;

        let mut file = File::create(&self.fname)?;
        file.write_all(&data)?;

        Ok(())
    }

    pub fn load(data_path: &PathBuf) -> Result<Self> {
        let fname = data_path.clone().join(STATE_FNAME);
        if fname.exists() {
            let mut data = Vec::new();

            let mut file = File::open(&fname)?;
            file.read_to_end(&mut data)?;

            let dirstack = json::from_slice(&data)?;
            Ok(Self {
                fname,
                dirstack,
            })
        } else {
            Ok(Self::new(fname))
        }
    }

    fn new(fname: PathBuf) -> Self {
        Self {
            fname,
            dirstack: Vec::new(),
        }
    }
}

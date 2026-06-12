use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub workspace: PathBuf,
}

impl Config {
    pub fn cards_dir(&self) -> PathBuf {
        self.workspace.join("cards")
    }

    pub fn scratch_dir(&self) -> PathBuf {
        self.workspace.join("scratch")
    }

    pub fn index_path(&self) -> PathBuf {
        self.workspace.join("index.json")
    }
}

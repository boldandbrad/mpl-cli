use super::Title;
use crate::util::fs::create_profile_dir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub config: MplConfig,
    pub stashes: Vec<Stash>,
}

impl Profile {
    pub fn load(name: String) -> Profile {
        Profile::new()
    }
}

pub fn create_profile() {}

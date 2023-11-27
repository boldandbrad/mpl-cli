// use super::Title;
use super::Profile;
use crate::util::fs::{create_dir, PROFILE_STASH_DIR_NAME, STASH_STATE_DIR_NAME};
use serde::{Deserialize, Serialize};

pub static DEFAULT_STASH_NAME: &str = "collection";

#[derive(Debug, Serialize, Deserialize)]
pub struct Stash {
    pub name: String,
    pub states: Vec<StashState>,
}

impl Stash {
    pub fn new(name: Option<&String>) -> Stash {
        Stash {
            name: name.unwrap_or(&DEFAULT_STASH_NAME.to_string()).to_owned(),
            states: vec![],
        }
    }

    pub fn load(name: String) -> Stash {
        Stash {
            name,
            // TODO: load states
            states: vec![],
        }
    }

    pub fn save(&self, profile: &Profile) {
        // ensure stash dir exists
        let stash_dir = &profile
            .get_dir()
            .join(PROFILE_STASH_DIR_NAME)
            .join(&self.name);
        create_dir(stash_dir);

        // ensure stash state dir exists
        let stash_state_dir = &stash_dir.join(STASH_STATE_DIR_NAME);
        create_dir(stash_state_dir);

        // TODO: save stash state to the file system
    }
    // pub fn get_current_state() -> StashState {}

    // pub fn get_previous_state() -> StashState {}

    // pub fn get_state_by_date() -> StashState {}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StashState {
    pub timestamp: String,
    pub title_ids: Vec<u32>,
}

impl StashState {
    // pub fn new() -> StashState {
    //     StashState {
    //         timestamp: "dfsd".to_string(),
    //         title_ids: vec![],
    //     }
    // }

    // pub fn save(&self) {}
    // pub fn get_titles() -> Vec<Title> {}
}

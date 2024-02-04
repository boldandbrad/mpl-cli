use super::Profile;
use crate::util::fs::{
    create_dir, delete_dir, get_profiles_state_dir, read_stash_state_file, write_stash_state_file,
    PROFILE_STASH_DIR_NAME, STASH_STATE_FILE_NAME,
};
use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub static DEFAULT_STASH_NAME: &str = "collection";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stash {
    pub name: String,
    pub state: StashState,
}

impl PartialEq for Stash {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Stash {}

impl Stash {
    pub fn new(name: Option<&String>) -> Stash {
        Stash {
            name: name.unwrap_or(&DEFAULT_STASH_NAME.to_string()).to_owned(),
            state: StashState::new(),
        }
    }

    pub fn load(name: &String, profile_name: &String) -> Stash {
        let stash_state_file = get_profiles_state_dir()
            .join(profile_name)
            .join(PROFILE_STASH_DIR_NAME)
            .join(name)
            .join(STASH_STATE_FILE_NAME);
        let stash_state = read_stash_state_file(stash_state_file);
        Stash {
            name: name.to_string(),
            state: stash_state,
        }
    }

    pub fn save(&mut self, profile_name: &String) {
        let stash_state_dir = get_profiles_state_dir()
            .join(profile_name)
            .join(PROFILE_STASH_DIR_NAME)
            .join(&self.name);

        create_dir(&stash_state_dir);

        self.state.timestamp = Utc::now();

        // write stash state file
        let _ = &self.state.save(&stash_state_dir);
    }

    pub fn delete(&self, profile: &Profile) {
        let stash_state_dir = &&profile
            .get_state_dir()
            .join(PROFILE_STASH_DIR_NAME)
            .join(&self.name);
        delete_dir(stash_state_dir)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StashState {
    pub timestamp: DateTime<Utc>,
    pub title_ids: Vec<u32>,
}

impl StashState {
    pub fn new() -> StashState {
        StashState {
            timestamp: Utc::now(),
            title_ids: vec![],
        }
    }

    pub fn save(&mut self, stash_state_dir: &Path) {
        self.timestamp = Utc::now();
        let stash_state_file = stash_state_dir.join(STASH_STATE_FILE_NAME);
        write_stash_state_file(self, stash_state_file);
    }

    pub fn add_title(&mut self, title_id: u32) {
        self.title_ids.push(title_id);
    }

    pub fn drop_title(&mut self, title_id: u32) {
        let index = self.title_ids.iter().position(|&t| t == title_id).unwrap();
        self.title_ids.remove(index);
    }

    // pub fn get_titles() -> Vec<Title> {}
}

// use super::{Config, Stash};
// use crate::util::config::get_default_config;
// use crate::util::fs::{get_profile_config, get_profile_stashes};
use crate::util::fs::create_profile_dir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    // pub config: Config,
    // pub stashes: Vec<Stash>,
}

impl Profile {
    // create a new profile instance with default values
    pub fn new(name: &String) -> Profile {
        Profile {
            name: name.to_owned(),
            // config: get_default_config(),
        }
    }

    // initialize new profile in the fs
    pub fn init(&self) {
        create_profile_dir(&self.name);
    }

    // load an existing profile
    pub fn load(name: String) -> Profile {
        // let profile_stashes = get_profile_stashes();
        Profile {
            name,
            // config: get_profile_config(),
            // stashes: get_profile_stashes(),
        }
    }
}

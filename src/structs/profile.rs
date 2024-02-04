use super::{Config, Stash};
use crate::util::fs::{
    create_dir, delete_dir, get_dir_names, get_profiles_config_dir, get_profiles_state_dir,
    PROFILE_STASH_DIR_NAME,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub config: Config,
    pub stashes: Vec<Stash>,
}

impl PartialEq for Profile {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Profile {}

impl Profile {
    // create a new profile instance with default values
    pub fn new(name: &String) -> Profile {
        Profile {
            name: name.to_owned(),
            config: Config::default(),
            stashes: vec![Stash::new(None)],
        }
    }

    // load an existing profile
    pub fn load(name: &String) -> Profile {
        let profile_config_dir = get_profiles_config_dir().join(name);
        let profile_state_dir = get_profiles_state_dir().join(name);
        let stash_names = get_dir_names(&profile_state_dir.join(PROFILE_STASH_DIR_NAME));
        let stashes: Vec<Stash> = stash_names.iter().map(|x| Stash::load(x, name)).collect();
        Profile {
            config: Config::load(Some(profile_config_dir)),
            name: name.to_owned(),
            stashes,
        }
    }

    pub fn get_config_dir(&self) -> PathBuf {
        get_profiles_config_dir().join(&self.name)
    }

    pub fn get_state_dir(&self) -> PathBuf {
        get_profiles_state_dir().join(&self.name)
    }

    // save the profile to the file system
    pub fn save(&self) {
        // ensure profile dirs exist
        let profile_config_dir = &self.get_config_dir();
        create_dir(profile_config_dir);
        let profile_state_dir = &self.get_state_dir();
        let profile_stashes_dir = &profile_state_dir.join(PROFILE_STASH_DIR_NAME);
        create_dir(profile_stashes_dir);

        // save profile config to the file system
        self.config.save(Some(profile_config_dir));

        // save profile stashes to the file system
        for mut stash in self.stashes.clone() {
            stash.save(&self.name)
        }
    }

    pub fn delete(&self) {
        delete_dir(&self.get_config_dir());
        delete_dir(&self.get_state_dir());
    }
}

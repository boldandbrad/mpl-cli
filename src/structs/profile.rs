use super::{Config, Stash};
use crate::util::fs::{
    create_dir, delete_dir, get_dir_names, get_profiles_dir, PROFILE_STASH_DIR_NAME,
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
    pub fn load(name: String) -> Profile {
        let profile_dir = get_profiles_dir().join(&name);
        let stash_names = get_dir_names(&profile_dir.join(PROFILE_STASH_DIR_NAME));
        let stashes: Vec<Stash> = stash_names.iter().map(Stash::load).collect();
        Profile {
            config: Config::load(Some(profile_dir)),
            name,
            stashes,
        }
    }

    pub fn get_dir(&self) -> PathBuf {
        get_profiles_dir().join(&self.name)
    }

    // save the profile to the file system
    pub fn save(&self) {
        // ensure profile dirs exist
        let profile_dir = &self.get_dir();
        create_dir(profile_dir);
        let profile_stashes_dir = &profile_dir.join(PROFILE_STASH_DIR_NAME);
        create_dir(profile_stashes_dir);

        // save profile config to the file system
        self.config.save(Some(profile_dir));

        // save profile stashes to the file system
        for stash in &self.stashes {
            stash.save(self)
        }
    }

    pub fn delete(&self) {
        delete_dir(&self.get_dir())
    }
}

use crate::structs::{GlobalState, Profile, Stash};
use crate::util::fs::{get_dir_names, get_profiles_config_dir};
use anyhow::{anyhow, Result};
use regex::Regex;

pub fn create(new_stash_names: Vec<String>, profile: Option<String>) -> Result<()> {
    // load active profile
    let mut active_profile = Profile::load(&GlobalState::load().active_profile);

    // check if provided profile exists and use it
    match profile {
        None => {}
        Some(profile_name) => {
            if get_dir_names(&get_profiles_config_dir()).contains(&profile_name) {
                active_profile = Profile::load(&profile_name);
            } else {
                return Err(anyhow!("Profile '{}' doesn't exist.", profile_name));
            }
        }
    }

    // create stash(es)
    for stash_name in new_stash_names {
        // check if given stash name is valid format (a-zA-Z0-9_-)
        let valid_chars = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
        if !valid_chars.is_match(&stash_name) {
            println!("Warning: {} is not a valid stash name.", stash_name);
            continue;
        }

        // check if stash already exists in profile
        let profile_stash_names = active_profile.get_stash_names();
        if profile_stash_names.contains(&stash_name) {
            println!(
                "Warning: {} already exists in profile {}.",
                stash_name, active_profile.name
            );
            continue;
        }

        // create and save new stash
        let mut new_stash = Stash::new(Some(&stash_name));
        new_stash.save(&active_profile.name);
        println!(
            "Created stash {} in profile {}.",
            stash_name, active_profile.name
        );
    }
    Ok(())
}

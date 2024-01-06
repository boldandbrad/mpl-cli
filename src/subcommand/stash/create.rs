use crate::structs::{GlobalState, Profile, Stash};
use crate::util::fs::{get_dir_names, get_profiles_dir};
use anyhow::{anyhow, Result};

pub fn create(stash_names: Vec<String>, profile: Option<String>) -> Result<()> {
    // load active profile
    let mut active_profile = Profile::load(GlobalState::load().active_profile);

    // TODO: check provided stash names are valid (a-zA-Z0-9-_)

    // check if provided profile exists and use it
    match profile {
        None => {}
        Some(profile_name) => {
            if get_dir_names(&get_profiles_dir()).contains(&profile_name) {
                active_profile = Profile::load(profile_name.to_owned());
            } else {
                return Err(anyhow!("Error: Profile '{}' doesn't exist.", profile_name));
            }
        }
    }

    // create stash(es)
    for stash_name in stash_names {
        // TODO: check if stash(es) already exist in profile
        let new_stash = Stash::new(Some(&stash_name));
        new_stash.save(&active_profile);
        println!(
            "Created stash {} in profile {}.",
            stash_name, active_profile.name
        );
    }
    Ok(())
}

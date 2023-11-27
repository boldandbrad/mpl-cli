use crate::structs::{GlobalState, Profile, Stash};
use crate::util::fs::{get_dir_names, get_profiles_dir};

pub fn create(stash_names: Vec<String>, profile: Option<String>) {
    // load active profile
    let mut active_profile = Profile::load(GlobalState::load().active_profile);

    // TODO: check provided stash names are valid (a-zA-Z0-9-_)

    // check if provided profile exists and use it
    let mut profile_exists = true;
    match profile {
        None => {}
        Some(profile_name) => {
            if get_dir_names(&get_profiles_dir()).contains(&profile_name) {
                active_profile = Profile::load(profile_name.to_owned());
            } else {
                println!("Error: Profile '{}' doesn't exist.", profile_name);
                profile_exists = false;
            }
        }
    }

    // create stash(es)
    if profile_exists {
        for stash_name in stash_names {
            // TODO: check if stash(es) already exist in profile
            let new_stash = Stash::new(Some(&stash_name));
            new_stash.save(&active_profile);
            println!(
                "Created stash {} in profile {}.",
                stash_name, active_profile.name
            );
        }
    }
}

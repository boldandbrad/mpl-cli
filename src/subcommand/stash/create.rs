use crate::util::fs::{create_profile_stash_dir, get_profile_names};
use crate::util::state::get_mpl_state;

pub fn create(stash_names: Vec<String>, profile: Option<String>) {
    let mut active_profile = get_mpl_state().active_profile;

    // TODO: check provided stash names are valid (a-zA-Z0-9-_)

    // check if provided profile exists and use it
    if profile.is_some() {
        let provided_profile = &profile.expect("Could not parse profile value.");
        if get_profile_names().contains(provided_profile) {
            active_profile = provided_profile.to_owned();
        } else {
            println!("Error: Profile '{}' doesn't exist.", provided_profile);
            return;
        }
    }

    // TODO: check if stash(es) already exist in profile

    for stash_name in stash_names {
        create_profile_stash_dir(&active_profile, &stash_name);
        println!(
            "Created stash {} in profile {}.",
            stash_name, active_profile
        );
    }
}

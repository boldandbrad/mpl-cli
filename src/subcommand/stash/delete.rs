use crate::structs::{GlobalState, Profile, Stash};

pub fn delete(stash_names: Vec<String>, _force: Option<bool>) {
    // TODO: implement force/yes option
    // load active profile
    let active_profile = Profile::load(GlobalState::load().active_profile);
    for stash_name in stash_names {
        // check the stash exists in the profile
        let stash = Stash::load(&stash_name);
        if active_profile.stashes.contains(&stash) {
            stash.delete(&active_profile);
            println!(
                "Deleted stash {} in profile {}.",
                stash_name, active_profile.name
            );
        } else {
            println!(
                "Error: Stash {} already does not exist in profile {}.",
                stash_name, active_profile.name
            )
        }
    }
}

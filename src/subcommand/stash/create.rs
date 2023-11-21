use crate::util::fs::create_profile_stash_dir;
use crate::util::state::get_mpl_state;

pub fn create(stash_names: Vec<String>) {
    let mpl_state = get_mpl_state();
    let active_profile = mpl_state.active_profile;
    // TODO: check if stash(es) already exist
    // TODO: check provided stash names are valid (a-zA-Z0-9-_)
    if stash_names.len() > 1 {
        for stash_name in stash_names {
            create_profile_stash_dir(&active_profile, &stash_name);
            println!(
                "Created stash {} in profile {}.",
                stash_name, active_profile
            );
        }
    } else {
        let stash_name = &stash_names[0];
        create_profile_stash_dir(&active_profile, stash_name);
        println!(
            "Created stash {} in profile {}.",
            stash_name, active_profile
        );
    }
}

use crate::structs::{GlobalState, Profile};

pub fn list() {
    // load active profile
    let active_profile = Profile::load(GlobalState::load().active_profile);

    for stash in active_profile.stashes {
        println!("{}", stash.name);
    }
}

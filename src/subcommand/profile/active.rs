use crate::structs::{GlobalState, Profile};

pub fn active() {
    // load active profile
    let active_profile = Profile::load(GlobalState::load().active_profile);
    println!("{}", active_profile.name);
}

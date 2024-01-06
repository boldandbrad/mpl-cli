use crate::structs::{GlobalState, Profile};
use anyhow::Result;

pub fn list() -> Result<()> {
    // load active profile
    let active_profile = Profile::load(GlobalState::load().active_profile);

    for stash in active_profile.stashes {
        println!("{}", stash.name);
    }
    Ok(())
}

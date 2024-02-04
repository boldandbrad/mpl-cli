use crate::structs::{GlobalState, Profile};
use anyhow::Result;

pub fn delete(stash_names: Vec<String>, _force: bool) -> Result<()> {
    // TODO: implement force/yes option
    // load active profile
    let active_profile = Profile::load(&GlobalState::load().active_profile);
    for stash_name in stash_names {
        let mut found = false;
        for stash in &active_profile.stashes {
            if stash.name == stash_name {
                stash.delete(&active_profile);
                println!(
                    "Deleted stash {} in profile {}.",
                    stash_name, active_profile.name
                );
                found = true;
                break;
            }
        }
        if !found {
            println!(
                "Warning: Stash {} already does not exist in profile {}.",
                stash_name, active_profile.name
            )
        }
    }
    Ok(())
}

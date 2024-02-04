use crate::structs::{GlobalState, Profile};
use crate::util::bgg_api::get_item;
use anyhow::{anyhow, Result};

pub fn add(stash_name: String, bgg_ids: Vec<u32>) -> Result<()> {
    // println!("Add {:?} to {:?}", &bgg_ids, stash_name);
    // check if the stash exists
    let active_profile = Profile::load(&GlobalState::load().active_profile);
    let profile_stash_names = active_profile.get_stash_names();
    if !profile_stash_names.contains(&stash_name) {
        return Err(anyhow!("Stash '{}' does not exist.", stash_name));
    }

    for mut stash in active_profile.stashes.clone() {
        if stash.name == stash_name {
            for bgg_id in &bgg_ids {
                // check that the given bgg_id is valid
                if get_item(bgg_id.to_string()).is_err() {
                    println!("Warning: {:?} is not a valid BGG id.", bgg_id);
                    continue;
                };

                // add title to stash if it doesn't already exist
                if !stash.state.title_ids.contains(bgg_id) {
                    stash.state.add_title(*bgg_id);
                    println!("Added {:?} to {:?}", bgg_id, stash_name);
                } else {
                    println!("Warning: {:?} already exists in {:?}", bgg_id, stash_name);
                }
            }
            // save stash changes
            stash.save(&active_profile.name);
            // active_profile.save();
            break;
        }
    }
    // active_profile.save();

    // TODO: update cache?

    Ok(())
}

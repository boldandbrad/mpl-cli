use crate::structs::{GlobalState, Profile};
use anyhow::{anyhow, Result};

pub fn drop(stash_name: String, bgg_ids: Vec<u32>) -> Result<()> {
    // println!("Drop {:?} from {:?}", bgg_ids, stash_name);
    // check if the stash exists
    let active_profile = Profile::load(&GlobalState::load().active_profile);
    let profile_stash_names = active_profile.get_stash_names();
    if !profile_stash_names.contains(&stash_name) {
        return Err(anyhow!("Stash '{}' does not exist.", stash_name));
    }

    // TODO: add confirmation

    for mut stash in active_profile.stashes.clone() {
        if stash.name == stash_name {
            for bgg_id in &bgg_ids {
                // check if the title already exists in the stash
                if stash.state.title_ids.contains(bgg_id) {
                    // drop title from stash
                    stash.state.drop_title(*bgg_id);
                } else {
                    println!(
                        "Warning: {:?} already does not exist in {:?}",
                        bgg_id, stash_name
                    );
                }
            }
            // println!("{:?}", stash.state.title_ids);
            // save stash changes
            stash.save(&active_profile.name);
            // active_profile.save();
            break;
        }
    }

    // TODO: update cache?

    Ok(())
}

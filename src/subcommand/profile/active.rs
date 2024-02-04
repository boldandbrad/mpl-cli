use crate::structs::{GlobalState, Profile};
use anyhow::Result;

pub fn active() -> Result<()> {
    // load active profile
    let active_profile = Profile::load(&GlobalState::load().active_profile);
    Ok(println!("{}", active_profile.name))
}

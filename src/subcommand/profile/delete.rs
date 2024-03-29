use crate::structs::{GlobalState, Profile};
use crate::util::fs::get_profiles_config_dir;
use anyhow::{anyhow, Result};

pub fn delete(profile_name: String) -> Result<()> {
    // load active profile
    let active_profile = Profile::load(&GlobalState::load().active_profile);
    if profile_name == active_profile.name {
        Err(anyhow!("Cannot delete active profile"))
    } else if !get_profiles_config_dir().join(&profile_name).exists() {
        Err(anyhow!("Profile {} already does not exist.", profile_name))
    } else {
        let delete_profile = Profile::load(&profile_name);
        delete_profile.delete();
        Ok(println!("Deleted profile {}", delete_profile.name))
    }
}

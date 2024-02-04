use crate::structs::{GlobalState, Profile};
use crate::util::fs::{get_dir_names, get_profiles_config_dir, write_global_state_file};
use anyhow::{anyhow, Result};

pub fn create(profile_name: String, active: Option<bool>) -> Result<()> {
    // check if profile already exists
    let profile_names = get_dir_names(&get_profiles_config_dir());
    if profile_names.contains(&profile_name) {
        return Err(anyhow!("Profile '{}' already exists.", profile_name));
    }

    // create and initialize profile
    let new_profile = Profile::new(&profile_name);
    new_profile.save();

    // if active flag provided, make created profile the active profile
    if let Some(true) = active {
        let mut mpl_state = GlobalState::load();
        mpl_state.active_profile = profile_name;
        write_global_state_file(mpl_state);
    }
    Ok(())
}

use crate::util::fs::{create_profile_dir, get_profile_names, write_mpl_state};
use crate::util::get_mpl_state;

pub fn create(profile_name: String, active: Option<bool>) {
    // check if profile already exists
    // TODO: close with error exit code on error
    let profile_names = get_profile_names();
    if profile_names.contains(&profile_name) {
        println!("Error: Profile '{}' already exists.", profile_name);
        return;
    }

    // create profile
    create_profile_dir(&profile_name);

    // if active flag provided, make created profile the active profile
    if let Some(true) = active {
        let mut mpl_state = get_mpl_state();
        mpl_state.active_profile = profile_name;
        write_mpl_state(mpl_state);
    }
}

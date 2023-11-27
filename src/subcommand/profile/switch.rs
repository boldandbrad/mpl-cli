use crate::structs::GlobalState;
use crate::util::fs::{get_dir_names, get_profiles_dir, write_global_state_file};

pub fn switch(profile_name: String) {
    // check that the given profile exists
    if get_dir_names(&get_profiles_dir()).contains(&profile_name) {
        // switch active profile
        let mut mpl_state = GlobalState::load();
        mpl_state.active_profile = profile_name;
        write_global_state_file(mpl_state);
    } else {
        println!("Error: Profile '{}' doesn't exist.", profile_name);
    }
}

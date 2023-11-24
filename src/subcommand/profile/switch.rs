use crate::util::fs::{get_profile_names, write_mpl_state};
use crate::util::get_mpl_state;

pub fn switch(profile_name: String) {
    // check that the given profile exists
    if get_profile_names().contains(&profile_name) {
        // switch active profile
        let mut mpl_state = get_mpl_state();
        mpl_state.active_profile = profile_name;
        write_mpl_state(mpl_state);
    } else {
        println!("Error: Profile '{}' doesn't exist.", profile_name);
        return;
    }
}

use crate::structs::{GlobalState, Profile};
use crate::util::fs::get_profiles_dir;

pub fn delete(profile_name: String) {
    // load active profile
    let active_profile = Profile::load(GlobalState::load().active_profile);
    if profile_name == active_profile.name {
        println!("Error: cannot delete active profile");
    } else if !get_profiles_dir().join(&profile_name).exists() {
        println!("Error: profile {} already does not exist.", profile_name)
    } else {
        let delete_profile = Profile::load(profile_name);
        delete_profile.delete();
        println!("Deleted profile {}", delete_profile.name)
    }
}

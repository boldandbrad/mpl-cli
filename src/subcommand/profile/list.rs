use crate::util::fs::{get_dir_names, get_profiles_dir};

pub fn list() {
    // TODO: mark active profile
    // TODO: sort alphabetically
    let profile_names = get_dir_names(&get_profiles_dir());
    for name in profile_names {
        println!("{}", name);
    }
}

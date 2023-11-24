use crate::util::fs::get_profile_names;

pub fn list() {
    // TODO: mark active profile
    // TODO: sort alphabetically
    let profile_names = get_profile_names();
    for name in profile_names {
        println!("{}", name);
    }
}

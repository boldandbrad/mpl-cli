use crate::util::fs::{get_dir_names, get_profiles_dir};
use anyhow::Result;

pub fn list() -> Result<()> {
    // TODO: mark active profile
    // TODO: sort alphabetically
    let profile_names = get_dir_names(&get_profiles_dir());
    for name in profile_names {
        println!("{}", name);
    }
    Ok(())
}

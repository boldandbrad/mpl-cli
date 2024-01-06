use anyhow::{anyhow, Result};

pub fn rename(profile_name: String, new_name: String) -> Result<()> {
    println!("Rename profile {} to {}.", profile_name, new_name);
    Err(anyhow!("Not yet implemented."))
}

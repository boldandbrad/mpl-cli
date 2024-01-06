use anyhow::{anyhow, Result};

pub fn add(stash_name: String, bgg_ids: Vec<String>) -> Result<()> {
    println!("Add {:?} to {:?}", bgg_ids, stash_name);
    Err(anyhow!("Not yet implemented."))
}

use anyhow::{anyhow, Result};

pub fn drop(stash_name: String, bgg_ids: Vec<String>) -> Result<()> {
    println!("Drop {:?} from {:?}", bgg_ids, stash_name);
    Err(anyhow!("Not yet implemented."))
}

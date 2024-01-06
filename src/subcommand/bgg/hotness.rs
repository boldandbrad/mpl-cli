use crate::util::bgg_api::hot_items;
use anyhow::Result;

pub fn hotness() -> Result<()> {
    let response = hot_items();
    match response {
        Ok(titles) => {
            for title in titles {
                println!("{}\t{}", title.id, title.name)
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

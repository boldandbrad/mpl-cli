use crate::util::bgg_api::search_items;
use anyhow::Result;

pub fn search(query: String) -> Result<()> {
    let response = search_items(query);
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

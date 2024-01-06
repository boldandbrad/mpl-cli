use crate::util::bgg_api::get_item;
use anyhow::Result;

pub fn info(bgg_id: String) -> Result<()> {
    let response = get_item(bgg_id);
    Ok(println!("{:?}", response))
}

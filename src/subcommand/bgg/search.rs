use crate::util::bgg_api::search_items;
use anyhow::Result;

pub fn search(query: String) -> Result<()> {
    println!("Search for {:?}", query);
    let response = search_items(query);
    // let response = get_items(vec!["266192".to_string()]);
    Ok(println!("{:#?}", response))
}

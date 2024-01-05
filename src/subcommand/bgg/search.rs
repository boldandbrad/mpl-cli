use crate::util::bgg_api::{get_items, search_items};

pub fn search(query: String) {
    println!("Search for {:?}", query);
    let response = search_items(query);
    // let response = get_items(vec!["266192".to_string()]);
    println!("{:#?}", response);
}

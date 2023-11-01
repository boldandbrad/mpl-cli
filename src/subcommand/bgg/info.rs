use crate::util::bgg_api::get_item;

pub fn info(bgg_id: String) {
    let response = get_item(bgg_id);
    println!("{:?}", response);
}

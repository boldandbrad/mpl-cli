use crate::util::bgg_api::get_item;
use anyhow::Result;

pub fn info(bgg_id: String) -> Result<()> {
    let response = get_item(bgg_id);
    match response {
        Ok(title) => {
            // print out title info
            println!("{}\t{} ({})", title.id, title.name, title.year);
            println!("  Rating:\tNA");
            println!("  Rank: \tNA");
            println!("  Weight:\tNA");
            println!("  Players:\t{}-{}", title.min_players, title.max_players);
            println!("  Play Time:\t~{} Min", title.play_time);
            println!("  Min Age:\t{}", title.min_age);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

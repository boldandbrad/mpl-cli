use crate::util::bgg_api::get_item;
use anyhow::Result;

pub fn info(bgg_id: u32, verbose: bool) -> Result<()> {
    let response = get_item(bgg_id.to_string());
    match response {
        Ok(title) => {
            // print out title info
            println!("{}\t{} ({})", title.id, title.name, title.year);
            if let Some(stats) = title.stats {
                println!("  Rating:\t{:.2}", stats.rating);
                println!("  Rank: \t{}", stats.rank);
                println!("  Weight:\t{:.2}", stats.weight);
            }
            println!("  Players:\t{}-{}", title.min_players, title.max_players);
            println!("  Play Time:\t~{} Min", title.play_time);
            println!("  Min Age:\t{}", title.min_age);
            if verbose {
                println!("  Description:\t{}", title.description);
                if let Some(credits) = title.credits {
                    println!("  Designer(s):\t{}", credits.designers.join(", "));
                    println!("  Artist(s):\t{}", credits.artists.join(", "));
                    println!("  Publisher(s):\t{}", credits.publishers.join(", "));
                }
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

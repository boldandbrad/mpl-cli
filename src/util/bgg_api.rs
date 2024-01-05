use crate::structs::Title;
use reqwest::{self, StatusCode};
use std::{thread, time};
use xmltree::Element;

static BGG_API2_BASE_URL: &str = "https://boardgamegeek.com/xmlapi2/";
static BGG_BOARDGAME_TYPE: &str = "boardgame";
static BGG_EXPANSION_TYPE: &str = "boardgameexpansion";

fn api_get(
    endpoint: &str,
    params: Vec<(&str, &str)>,
) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let mut base_url: String = BGG_API2_BASE_URL.to_string();
    base_url.push_str(endpoint);
    let url = reqwest::Url::parse_with_params(base_url.as_str(), &params)?;
    loop {
        let response = reqwest::blocking::get(url.clone())?;
        match response.status() {
            StatusCode::OK => {
                // successful response value
                return Ok(response.text()?);
            }
            StatusCode::ACCEPTED => {
                // wait and ping again for response
                thread::sleep(time::Duration::from_secs(12));
            }
            StatusCode::TOO_MANY_REQUESTS => {
                // rate limit reached
                return Err(String::from(
                    "BoardGameGeek API rate limit reached. Please try again later.",
                )
                .into());
            }
            s => {
                // unknown api response
                let err_msg = format!("Unknown API Response ({:?}) - {:?}", s, response.text());
                return Err(err_msg.into());
            }
        }
    }
}

pub fn get_items(bgg_ids: Vec<String>) -> Vec<Title> {
    let bgg_ids_binding = bgg_ids.join(",");
    let types_binding = format!("{},{}", BGG_BOARDGAME_TYPE, BGG_EXPANSION_TYPE);
    let params = vec![
        ("id", bgg_ids_binding.as_str()),
        ("type", types_binding.as_str()),
        ("stats", "1"),
    ];
    // serialize returned xml to return vector of Titles?
    let xml_str = api_get("thing", params).unwrap();
    // println!("{}", xml_str);
    let items_element = Element::parse(xml_str.as_bytes()).unwrap();
    let mut titles: Vec<Title> = vec![];
    for child in items_element.children {
        // println!("{:#?}", child);
        let title = Title::from(child.as_element().unwrap());
        titles.push(title);
    }
    titles
}

pub fn get_item(bgg_id: String) -> Title {
    let titles = get_items(vec![bgg_id]);
    titles.into_iter().nth(0).unwrap()
}

pub fn search_items(query: String) -> Vec<Title> {
    let params = vec![("type", BGG_BOARDGAME_TYPE), ("query", query.as_str())];
    let xml_str = api_get("search", params).unwrap();
    // println!("{}", xml_str);
    let items_element = Element::parse(xml_str.as_bytes()).unwrap();
    let mut titles: Vec<Title> = vec![];
    for child in items_element.children {
        println!("{:#?}", child);
        let title = Title::from(child.as_element().unwrap());
        titles.push(title);
    }
    titles
}

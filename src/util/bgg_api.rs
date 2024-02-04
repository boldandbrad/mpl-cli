use crate::structs::Title;
use crate::util::xml::get_element_ids;
use anyhow::{anyhow, Result};
use reqwest::{self, StatusCode};
use std::{thread, time};
use xmltree::Element;

static BGG_XML_API2_BASE_URL: &str = "https://boardgamegeek.com/xmlapi2/";
static BGG_BOARDGAME_TYPE: &str = "boardgame";
static BGG_EXPANSION_TYPE: &str = "boardgameexpansion";

fn xml_api2_get(endpoint: &str, params: Vec<(&str, &str)>) -> Result<std::string::String> {
    let mut base_url: String = BGG_XML_API2_BASE_URL.to_string();
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
                return Err(anyhow!(
                    "BoardGameGeek API rate limit reached. Please try again later.",
                ));
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                // api is unavailable due to upgrades or otherwise
                return Err(anyhow!(
                    "BoardGameGeek API is currently down for maintenance. Please try again later.",
                ));
            }
            s => {
                // unknown api response
                return Err(anyhow!(
                    "Unknown API Response ({:?}) - {:?}",
                    s,
                    response.text()
                ));
            }
        }
    }
}

pub fn get_items(bgg_ids: Vec<String>) -> Result<Vec<Title>> {
    // build and perform get request
    let bgg_ids_binding = bgg_ids.join(",");
    let types_binding = format!("{},{}", BGG_BOARDGAME_TYPE, BGG_EXPANSION_TYPE);
    let params = vec![
        ("id", bgg_ids_binding.as_str()),
        ("type", types_binding.as_str()),
        ("stats", "1"),
    ];
    let api_get_result = xml_api2_get("thing", params);
    let xml_str = match api_get_result {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    // serialize result items into a list of titles
    let mut titles: Vec<Title> = vec![];
    for child in Element::parse(xml_str.as_bytes())?.children {
        let title = Title::from(child.as_element().unwrap());
        titles.push(title);
    }
    Ok(titles)
}

pub fn get_item(bgg_id: String) -> Result<Title> {
    let items_result = get_items(vec![bgg_id.clone()]);
    match items_result {
        Ok(titles) => {
            if !titles.is_empty() {
                Ok(titles.into_iter().next().unwrap())
            } else {
                Err(anyhow!("Item {:?} not found", bgg_id))
            }
        }
        Err(e) => Err(e),
    }
}

pub fn hot_items() -> Result<Vec<Title>> {
    let params = vec![("type", BGG_BOARDGAME_TYPE)];
    let api_hot_result = xml_api2_get("hot", params);
    let xml_str = match api_hot_result {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    // get and return full item details
    let title_ids = get_element_ids(&Element::parse(xml_str.as_bytes())?.children);
    get_items(title_ids)
}

pub fn search_items(query: String) -> Result<Vec<Title>> {
    // build and perform search request
    let params = vec![("type", BGG_BOARDGAME_TYPE), ("query", query.as_str())];
    let api_search_result = xml_api2_get("search", params);
    let xml_str = match api_search_result {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    // get and return full item details
    let title_ids = get_element_ids(&Element::parse(xml_str.as_bytes())?.children);
    get_items(title_ids)
}

use crate::util::xml::get_child_element_val;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::str::FromStr;
use xmltree::Element;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TitleType {
    BoardGame,
    Expansion,
}

impl FromStr for TitleType {
    type Err = ();

    fn from_str(input: &str) -> Result<TitleType, Self::Err> {
        match input {
            "boardgame" => Ok(TitleType::BoardGame),
            "boardgameexpansion" => Ok(TitleType::Expansion),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub id: u32,
    pub name: String,
    pub ttype: TitleType,
    pub year: u16,
    pub min_players: u8,
    pub max_players: u8,
    pub play_time: u16,
    pub min_age: u8,
    pub description: String,
    // pub stats: Option<TitleStats>,
    // pub credits: Option<TitleCredits>,
}

impl Title {}

impl Default for Title {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            ttype: TitleType::BoardGame,
            year: 0,
            min_players: 0,
            max_players: 0,
            play_time: 0,
            min_age: 0,
            description: "".to_string(),
        }
    }
}

impl From<&Element> for Title {
    fn from(element: &Element) -> Self {
        let item_attrs = &element.attributes;
        let item_description = element
            .get_child("description")
            .unwrap()
            .get_text()
            .unwrap()
            .to_string();
        Self {
            id: item_attrs.get("id").unwrap().parse::<u32>().unwrap(),
            name: get_child_element_val(element, "name"),
            ttype: TitleType::from_str(item_attrs.get("type").unwrap()).unwrap(),
            year: get_child_element_val(element, "yearpublished")
                .parse::<u16>()
                .unwrap(),
            min_players: get_child_element_val(element, "minplayers")
                .parse::<u8>()
                .unwrap(),
            max_players: get_child_element_val(element, "maxplayers")
                .parse::<u8>()
                .unwrap(),
            play_time: get_child_element_val(element, "playingtime")
                .parse::<u16>()
                .unwrap(),
            min_age: get_child_element_val(element, "minage")
                .parse::<u8>()
                .unwrap(),
            description: html_escape::decode_html_entities(&item_description).to_string(),
        }
    }
}

impl PartialEq for Title {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Title {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleStats {
    pub rating: f32,
    pub weight: f32,
    pub rank: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleCredits {
    pub designers: Vec<String>,
    pub artists: Vec<String>,
    pub publishers: Vec<String>,
}

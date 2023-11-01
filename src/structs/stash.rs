use super::Title;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stash {
    pub name: String,
    pub state: StashState,
    pub data: StashData,
}

impl Stash {}

#[derive(Debug, Serialize, Deserialize)]
pub struct StashState {
    pub active_ids: Vec<u32>,
    pub to_add_ids: Vec<u32>,
    pub to_drop_ids: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StashData {
    pub updated_date: String,
    pub items: Vec<Title>,
}

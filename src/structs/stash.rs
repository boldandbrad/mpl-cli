// use super::Title;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stash {
    pub name: String,
    pub state: StashState,
}

impl Stash {
    // pub fn get_current_state() -> StashState {}

    // pub fn get_previous_state() -> StashState {}

    // pub fn get_state_by_date() -> StashState {}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StashState {
    pub timestamp: String,
    pub title_ids: Vec<u32>,
}

impl StashState {
    // pub fn get_titles() -> Vec<Title> {}
}

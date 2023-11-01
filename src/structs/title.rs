use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub id: u32,
    pub name: String,
    pub ttype: String,
    pub year: u16,
    pub rating: f32,
    pub weight: f32,
    pub rank: u16,
    pub min_players: u8,
    pub max_players: u8,
    pub play_time: u16,
    pub min_age: u8,
    pub description: String,
}

impl Title {
    // fn from_dict() -> Item {
    //     println!("Item from dict not yet implemented");
    // }
}

impl PartialEq for Title {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Title {}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {}

impl Config {
    // get default config
    pub fn default() -> Config {
        Config {}
    }
}

use crate::util::fs::{read_config_file, write_config_file};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {}

impl Config {
    // load an existing config
    pub fn load(conf_location: Option<PathBuf>) -> Config {
        read_config_file(conf_location)
    }

    // save config to file system
    pub fn save(&self, conf_location: Option<&PathBuf>) {
        write_config_file(self, conf_location);
    }
}

// impl Default for Config {
//     // get default config
//     fn default() -> Self {
//         Self {}
//     }
// }

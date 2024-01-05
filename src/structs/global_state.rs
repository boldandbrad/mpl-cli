use serde::{Deserialize, Serialize};

use crate::util::fs::{get_mpl_state_file, get_system_user, read_toml_file};

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalState {
    pub active_profile: String,
}

impl GlobalState {
    // load global state
    pub fn load() -> GlobalState {
        let toml_str = read_toml_file(&get_mpl_state_file());
        toml::from_str(&toml_str).expect("Could not decode toml state string.")
    }
}

impl Default for GlobalState {
    // get default global state
    fn default() -> Self {
        Self {
            active_profile: get_system_user(),
        }
    }
}

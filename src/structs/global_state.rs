use serde::{Deserialize, Serialize};

use crate::util::fs::{get_mpl_state_file, get_system_user, read_toml};

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalState {
    pub active_profile: String,
}

impl GlobalState {
    // load global state
    pub fn load() -> GlobalState {
        let toml_str = read_toml(&get_mpl_state_file());
        toml::from_str(&toml_str).expect("Could not decode toml state string.")
    }

    // get default global state
    pub fn default() -> GlobalState {
        GlobalState {
            active_profile: get_system_user(),
        }
    }
}

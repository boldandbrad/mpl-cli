use crate::structs::MplState;

use super::{get_mpl_state_file, get_system_user, read_toml};

pub fn get_mpl_default_state() -> MplState {
    MplState {
        active_profile: get_system_user(),
    }
}

pub fn get_mpl_state() -> MplState {
    let toml_str = read_toml(&get_mpl_state_file());
    toml::from_str(&toml_str).expect("Could not decode toml state string.")
}

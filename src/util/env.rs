use std::env;

// pub enum EnvVar {
//     MplConfigHome,
//     XdgCacheHome,
//     XdgConfigHome,
//     XdgDataHome,
//     XdgStateHome,
// }

// impl EnvVar {
//     fn as_str(&self) -> &'static str {
//         match self {
//             EnvVar::MplConfigHome => "MPL_CONFIG_HOME",
//             EnvVar::XdgCacheHome => "XDG_CACHE_HOME",
//             EnvVar::XdgConfigHome => "XDG_CONFIG_HOME",
//             EnvVar::XdgDataHome => "XDG_DATA_HOME",
//             EnvVar::XdgStateHome => "XDG_STATE_HOME",
//         }
//     }

//     fn get_val(&self) -> Option<String> {
//         match env::var(self.as_str()) {
//             Ok(val) => Some(val),
//             _ => None,
//         }
//     }
// }

pub fn get_system_user() -> String {
    whoami::username()
}

pub fn get_os() -> String {
    env::consts::OS.to_string()
}

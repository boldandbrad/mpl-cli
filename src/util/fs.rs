use crate::util::env::{get_os, get_system_user};
use std::path::{Path, PathBuf};

use crate::structs::{Config, GlobalState, Profile};

/*
Intended file structure:

Config directory
$XDG_CONFIG_HOME/mpl/
    config.toml
    profiles/
        profile-1/
            config.toml
        profile-2/
            config.toml


    config.toml


State directory
$XDG_STATE_HOME/mpl/
    state.toml

Data directory?
$XDG_DATA_HOME/mpl/

Cache directory
$XDG_CACHE_HOME/mpl/
    title_data/
        2024/
            2024-01/
                2024-01-01.ron
                ...
*/

static MPL_ROOT_DIR_NAME: &str = "mpl";
static PROFILES_DIR_NAME: &str = "profiles";
pub static PROFILE_STASH_DIR_NAME: &str = "stashes";
pub static STASH_STATE_DIR_NAME: &str = "state";

pub static CONFIG_FILE_NAME: &str = "config.toml";
static STATE_FILE_NAME: &str = "state.toml";

fn get_config_dir() -> PathBuf {
    let mpl_root_dir: String = MPL_ROOT_DIR_NAME.to_string();
    let user: String = get_system_user();
    match get_os().as_str() {
        "macos" => PathBuf::from(format!("/Users/{}/.config/{}", user, mpl_root_dir)),
        "linux" => PathBuf::from(format!("/home/{}/.config/{}", user, mpl_root_dir)),
        "windows" => {
            let mut path = PathBuf::new().join(r"C:\").join("Users");
            path.push(user);
            path.push(mpl_root_dir);
            path
        }
        &_ => PathBuf::from(mpl_root_dir),
    }
}

pub fn get_profiles_dir() -> PathBuf {
    get_config_dir().join(PROFILES_DIR_NAME)
}

pub fn get_mpl_state_file() -> PathBuf {
    get_config_dir().join(STATE_FILE_NAME)
}

pub fn get_global_config_file() -> PathBuf {
    get_config_dir().join(CONFIG_FILE_NAME)
}

pub fn create_dir(dir_path: &Path) {
    std::fs::create_dir_all(dir_path).expect("Could not create directories");
}

pub fn delete_dir(dir_path: &Path) {
    if dir_path.exists() {
        std::fs::remove_dir_all(dir_path).expect("Could not delete directories");
    }
}

pub fn read_toml_file(file_path: &Path) -> String {
    std::fs::read_to_string(file_path).expect("Could not read toml file.")
}

pub fn write_file(file_path: &Path, file_contents: String) {
    std::fs::write(file_path, file_contents).expect("Could not write file.");
}

pub fn read_config_file(conf_location: Option<PathBuf>) -> Config {
    let conf_str = read_toml_file(
        &conf_location
            .unwrap_or(get_config_dir())
            .join(CONFIG_FILE_NAME),
    );
    toml::from_str(&conf_str).expect("Could not decode toml string.")
}

pub fn write_config_file(config: &Config, conf_location: Option<&PathBuf>) {
    let toml_conf = toml::to_string(config).expect("Could not encode toml config value.");
    write_file(
        &conf_location
            .unwrap_or(&get_config_dir())
            .join(CONFIG_FILE_NAME),
        toml_conf,
    );
}

pub fn write_global_state_file(state: GlobalState) {
    let toml_state = toml::to_string(&state).expect("Could not encode toml state value.");
    write_file(&get_mpl_state_file(), toml_state);
}

pub fn get_dir_names(dir_location: &PathBuf) -> Vec<String> {
    let dir_paths: Vec<PathBuf> = std::fs::read_dir(dir_location)
        .unwrap()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|r| r.is_dir())
        .collect();
    let mut dir_names = vec![];
    for path in &dir_paths {
        dir_names.push(path.file_name().unwrap().to_str().unwrap().to_string());
    }
    dir_names
}

pub fn check_fs() {
    // create mpl dir if it does not exist
    let mpl_dir: PathBuf = get_config_dir();
    create_dir(&mpl_dir);

    // create global config and state files if they do not exist
    if !get_global_config_file().exists() {
        write_config_file(&Config::default(), None);
    }
    if !get_mpl_state_file().exists() {
        write_global_state_file(GlobalState::default());
    }

    // ensure profiles dir exists
    let profiles_dir = get_profiles_dir();
    create_dir(&profiles_dir);

    // create default profile if no profiles exist
    let profile_dirs: Vec<PathBuf> = std::fs::read_dir(&profiles_dir)
        .unwrap()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|r| r.is_dir())
        .collect();
    if profile_dirs.is_empty() {
        let default_profile_name = get_system_user();
        let default_profile = Profile::new(&default_profile_name);
        default_profile.save();
    }
}

use std::env;
use std::path::{Path, PathBuf};

use crate::structs::{MplConfig, MplState};

use super::{get_default_config, get_mpl_default_state};

static MPL_ROOT_DIR_NAME: &str = ".mplrs";

static PROFILES_DIR_NAME: &str = "profiles";
static PROFILE_STASH_DIR_NAME: &str = "stash";

static STASH_DATA_DIR_NAME: &str = "data";

static DEFAULT_STASH_NAME: &str = "collection";

static CONFIG_FILE_NAME: &str = "config.toml";
static STATE_FILE_NAME: &str = "state.toml";

pub fn get_system_user() -> String {
    whoami::username()
}

pub fn get_mpl_dir() -> PathBuf {
    let mpl_root_dir: String = MPL_ROOT_DIR_NAME.to_string();
    let user: String = get_system_user();
    let os: &str = env::consts::OS;
    match os {
        "macos" => PathBuf::from(format!("/Users/{}/{}", user, mpl_root_dir)),
        "linux" => PathBuf::from(format!("/home/{}/{}", user, mpl_root_dir)),
        "windows" => {
            let mut path = PathBuf::new().join(r"C:\").join("Users");
            path.push(user);
            path.push(mpl_root_dir);
            path
        }
        &_ => PathBuf::from(mpl_root_dir),
    }
}

pub fn get_mpl_state_file() -> PathBuf {
    get_mpl_dir().join(STATE_FILE_NAME)
}

pub fn get_mpl_config_file() -> PathBuf {
    get_mpl_dir().join(CONFIG_FILE_NAME)
}

pub fn create_dirs(dir_path: &Path) {
    std::fs::create_dir_all(dir_path).expect("Could not create directories");
}

pub fn create_profile_dir(profile_name: &String) {
    let profiles_dir: PathBuf = get_mpl_dir().join(PROFILES_DIR_NAME);
    // create profile dir
    create_dirs(&profiles_dir.join(profile_name));
    // create profile stashes dir and default stash
    create_profile_stash_dir(&profile_name, &DEFAULT_STASH_NAME.to_string());
    // TODO: create profile config file
}

// pub fn write_toml(file_path: &Path, )

pub fn write_mpl_config(config: MplConfig) {
    let toml_conf = toml::to_string(&config).expect("Could not encode toml config value.");
    std::fs::write(get_mpl_config_file(), toml_conf).expect("Could not write to config file.");
}

pub fn write_mpl_state(state: MplState) {
    let toml_state = toml::to_string(&state).expect("Could not encode toml state value.");
    std::fs::write(get_mpl_state_file(), toml_state).expect("Could not write to state file.");
}

pub fn read_toml(file_path: &Path) -> String {
    std::fs::read_to_string(file_path).expect("Could not read state file.")
}

pub fn create_profile_stash_dir(profile_name: &String, stash_name: &String) {
    // create stash dir
    let profiles_dir: PathBuf = get_mpl_dir().join(PROFILES_DIR_NAME);
    let profile_stash_dir: PathBuf = profiles_dir
        .clone()
        .join(profile_name)
        .join(PROFILE_STASH_DIR_NAME)
        .join(stash_name);
    create_dirs(&profile_stash_dir);

    // create stash sub dirs
    create_dirs(&profile_stash_dir.clone().join(STASH_DATA_DIR_NAME));
}

pub fn check_fs() {
    // create mpl dir if it does not exist
    let mpl_dir: PathBuf = get_mpl_dir();
    create_dirs(&mpl_dir);

    // create config and state files if it does not exist
    write_mpl_config(get_default_config());
    write_mpl_state(get_mpl_default_state());

    // create profiles dir if it does not exist
    let profiles_dir: PathBuf = mpl_dir.join(PROFILES_DIR_NAME);
    create_dirs(&profiles_dir);

    // create default profile dir if no profiles exist
    let profile_dirs: Vec<PathBuf> = std::fs::read_dir(&profiles_dir)
        .unwrap()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|r| r.is_dir())
        .collect();
    if profile_dirs.is_empty() {
        let default_profile_name = get_system_user();
        create_profile_dir(&default_profile_name);
    }
}

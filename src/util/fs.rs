use std::env;
use std::path::{Path, PathBuf};

static PROFILES_DIR_NAME: &str = "profiles";
static PROFILE_STASH_DIR_NAME: &str = "stash";

static STASH_DATA_DIR_NAME: &str = "data";

static DEFAULT_STASH_NAME: &str = "collection";

fn get_system_user() -> String {
    whoami::username()
}

pub fn get_mpl_dir() -> PathBuf {
    let mpl_root_dir: String = ".mplrs".to_string();
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

// pub fn get_mpl_state_file() -> PathBuf {
//     get_mpl_dir().join("state.yml")
// }

pub fn create_dirs(dir_path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dir_path)?;
    Ok(())
}

pub fn create_profile_dir(profile_name: &String) {
    let profiles_dir: PathBuf = get_mpl_dir().join(PROFILES_DIR_NAME);
    let _ = create_dirs(&profiles_dir.join(profile_name));
}

pub fn create_profile_stash_dir(profile_name: &String, stash_name: &String) {
    // create stash dir
    let profiles_dir: PathBuf = get_mpl_dir().join(PROFILES_DIR_NAME);
    let profile_stash_dir: PathBuf = profiles_dir
        .clone()
        .join(profile_name)
        .join(PROFILE_STASH_DIR_NAME)
        .join(stash_name);
    let _ = create_dirs(&profile_stash_dir);

    // create stash sub dirs
    let _ = create_dirs(&profile_stash_dir.clone().join(STASH_DATA_DIR_NAME));
}

pub fn check_fs() {
    // create mpl dir if it does not exist
    let mpl_dir: PathBuf = get_mpl_dir();
    let _ = create_dirs(&mpl_dir);

    // create profiles dir if it does not exist
    let profiles_dir: PathBuf = mpl_dir.clone().join(PROFILES_DIR_NAME);
    let _ = create_dirs(&profiles_dir);

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
        create_profile_stash_dir(&default_profile_name, &DEFAULT_STASH_NAME.to_string());
    }
}

use std::env;
use std::path::PathBuf;

pub fn get_mpl_dir() -> PathBuf {
    let mpl_root_dir: String = ".mplrs".to_string();
    let user: String = whoami::username();
    let os: &str = env::consts::OS;
    match os {
        "macos" => PathBuf::from(format!("/Users/{}/{}", user, mpl_root_dir)),
        "linux" => PathBuf::from(format!("/home/{}/{}", user, mpl_root_dir)),
        "windows" => {
            let mut path = PathBuf::new().join(r"C:\").join("Users");
            path.push(user);
            path.push(mpl_root_dir);
            return path;
        }
        &_ => PathBuf::from(mpl_root_dir),
    }
}

pub fn get_mpl_state_file() -> PathBuf {
    return get_mpl_dir().join("state.yml");
}

pub fn create_dirs(dir_pathbuf: &PathBuf) -> std::io::Result<()> {
    let dir_path: String = dir_pathbuf.clone().into_os_string().into_string().unwrap();
    std::fs::create_dir_all(dir_path)?;
    Ok(())
}

pub fn check_fs() {
    // create mpl dir if it does not exist
    let mpl_pathbuf: PathBuf = get_mpl_dir();
    let _ = create_dirs(&mpl_pathbuf);

    // create stash dir if it does not exist
    let col_pathbuf: PathBuf = mpl_pathbuf.clone().join("stashes");
    let _ = create_dirs(&col_pathbuf);

    // create stash sub dirs if they do not exist
    // let col_data_pathbuf: PathBuf = col_pathbuf.clone().join("data");
    // let _ = create_dirs(&col_data_pathbuf);
    // let col_state_pathbuf: PathBuf = col_pathbuf.clone().join("state");
    // let _ = create_dirs(&col_state_pathbuf);
}

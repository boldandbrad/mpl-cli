use semver::Version;

use crate::structs::mpl_state::MplState;
use crate::util::bgg_api::get_items;

fn test_write(fpath: String) -> std::io::Result<()> {
    let state = MplState {
        data_version: Version::parse("0.1.0").unwrap().to_string(),
    };
    println!("{:?}", state);
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(fpath)
        .expect("Couldn't open file");
    serde_yaml::to_writer(f, &state).unwrap();
    Ok(())
}

pub fn test() {
    // let fbuf: PathBuf = get_mpl_state_file();
    // let fpath: String = fbuf.clone().into_os_string().into_string().unwrap();
    // println!("{fpath}");
    // let _result = test_write(fpath);
    let mut bgg_ids = Vec::new();
    bgg_ids.push("123456".to_string());
    let response = get_items(bgg_ids);
    println!("{:?}", response);
}

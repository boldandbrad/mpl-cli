// use crate::util::fs::get_mpl_dir;

pub fn create(stash_names: Vec<String>) {
    println!("Create {:?}", stash_names);
    if stash_names.len() > 1 {
        println!("more than one")
    } else {
        println!("one")
    }
    // let val = get_mpl_dir();
    // println!("{}", val.into_os_string().into_string().unwrap())
}

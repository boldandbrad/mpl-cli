use crate::util::get_mpl_state;

pub fn active() {
    let mpl_state = get_mpl_state();
    let active_profile = mpl_state.active_profile;
    println!("{}", active_profile);
}

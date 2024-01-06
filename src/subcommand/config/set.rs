use anyhow::{anyhow, Result};

pub fn set(option_name: String, option_value: String) -> Result<()> {
    println!(
        "Set the value of given config option {:?}:{:?}",
        option_name, option_value
    );
    Err(anyhow!("Not yet implemented."))
}

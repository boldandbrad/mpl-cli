use anyhow::{anyhow, Result};

pub fn get(option_name: String) -> Result<()> {
    println!(
        "Get the current value of given config option {:?}",
        option_name
    );
    Err(anyhow!("Not yet implemented."))
}

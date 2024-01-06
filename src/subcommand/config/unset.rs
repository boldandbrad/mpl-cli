use anyhow::{anyhow, Result};

pub fn unset(option_name: String) -> Result<()> {
    println!(
        "Revert the value of the given option to its default: {:?}",
        option_name
    );
    Err(anyhow!("Not yet implemented."))
}

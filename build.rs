use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("Comptroller", "abi/comptroller.json")?
        .generate()?
        .write_to_file("src/abi/comptroller.rs")?;
    Ok(())
}

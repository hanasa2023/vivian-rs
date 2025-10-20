use color_eyre::{Result, eyre::Ok};
mod errors;

#[tokio::main]
async fn main() -> Result<()> {
    errors::install_hooks()?;

    Ok(())
}

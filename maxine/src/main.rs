pub mod error;

use miette::{Context, Result};
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("Setting up...");

    Err(error::MaxineError::Unknown).wrap_err("Failed to do some unknown thing.")?;
    Ok(())
}

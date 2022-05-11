use crate::services::logging;
use anyhow::Result;
use tracing::info;

mod services;

fn main() -> Result<()> {
    logging::setup()?;

    info!("Hello, world!");

    Ok(())
}

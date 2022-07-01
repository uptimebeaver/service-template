use anyhow::Result;
use std::env;
use tonic::transport::Server;
use tracing::{info, warn};

use template_lib::services::greeter::GreeterService; // TODO: Change me

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    utils::logging::setup(env::var("RUST_LOG")?)?;

    info!("starting service.");

    let port = env::var("PORT")?;
    let addr = format!("[::]:{}", port).parse()?;

    Server::builder()
        .add_service(GreeterService::create_server()) // TODO: Change me
        .serve(addr)
        .await?;

    warn!("quitting service");

    Ok(())
}

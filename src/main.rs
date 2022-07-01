use anyhow::Result;
use tonic::transport::Server;
use tracing::{info, warn};

use template_lib::services::greeter::GreeterService; // TODO: Change me

#[tokio::main]
async fn main() -> Result<()> {
    utils::logging::setup()?;

    info!("starting service.");

    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(GreeterService::create_server()) // TODO: Change me
        .serve(addr)
        .await?;

    warn!("quitting service");

    Ok(())
}

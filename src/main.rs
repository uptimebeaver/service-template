use color_eyre::eyre::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::env;
use tonic::transport::Server;
use tracing::{info, warn};
use tracing_subscriber::prelude::*;

use service_template::services::greeter::GreeterService; // TODO: Change me

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    #[cfg(debug_assertions)]
    dotenv::dotenv()?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("setting up database");

    let conn = Database::connect(env::var("DATABASE_URL")?).await?;
    Migrator::up(&conn, None).await?;

    info!("finished setting up database");

    info!("starting service.");

    let port = env::var("PORT")?;
    let addr = format!("[::]:{}", port).parse()?;

    Server::builder()
        .add_service(GreeterService::create_server(conn)) // TODO: Change me
        .serve(addr)
        .await?;

    warn!("quitting service");

    Ok(())
}

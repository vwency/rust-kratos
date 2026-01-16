use crate::application::config::Config;
use crate::infrastructure::adapters::graphql::schema::create_schema;
use crate::infrastructure::adapters::http::server;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::EnvFilter;

pub async fn run() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .init();

    info!("Starting application...");

    info!("Loading configuration...");
    let config = Config::from_env().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to load config: {}", e),
        )
    })?;

    info!("Creating GraphQL schema...");
    let schema = Arc::new(create_schema(&config));

    server::start(schema, config.server).await
}

use crate::application::bootstrap::config::Config;
use crate::infrastructure::adapters::http::server;
use crate::infrastructure::di::container::AppContainer;
use crate::presentation::api::graphql::schema::create_schema;
use std::sync::Arc;
use tokio::signal;
use tracing::info;
use tracing_subscriber::EnvFilter;

pub async fn run() -> anyhow::Result<()> {
    init_tracing()?;
    info!("Starting application...");

    info!("Loading configuration...");
    let config = Config::from_env()?;

    info!("Initializing dependency injection container...");
    let container = AppContainer::new(&config)?;

    info!("Creating GraphQL schema...");
    let schema = Arc::new(create_schema(&container));

    let server_handle = tokio::spawn(server::start(schema, config.server));

    shutdown_signal().await;
    info!("Shutdown signal received, starting graceful shutdown...");

    match server_handle.await {
        Ok(result) => result,
        Err(e) => Err(anyhow::anyhow!("Server task panicked: {}", e)),
    }
}

fn init_tracing() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .try_init()
        .map_err(|e| anyhow::anyhow!("Failed to initialize tracing subscriber: {}", e))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C signal");
        },
        _ = terminate => {
            info!("Received SIGTERM signal");
        },
    }
}

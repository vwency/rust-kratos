use crate::application::bootstrap::config::Config;
use crate::infrastructure::adapters::http::server;
use crate::infrastructure::di::container::AppContainer;
use crate::presentation::api::graphql::schema::create_schema;
use std::sync::Arc;
use tokio::signal;
use tracing::info;
use tracing_subscriber::EnvFilter;

pub async fn run() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    init_tracing(&config.server.log_level)?;
    info!("Starting application...");
    info!("Initializing dependency injection container...");
    let container = AppContainer::new(&config)?;
    info!("Creating GraphQL schema...");
    let schema = Arc::new(create_schema(&container));

    tokio::select! {
        result = server::start(
            schema,
            config.server,
            container.hydra_client.clone(),
            container.kratos_client.clone(),
        ) => {
            if let Err(e) = result {
                return Err(anyhow::anyhow!("Server error: {}", e));
            }
        },
        _ = shutdown_signal() => {
            info!("Shutdown signal received, starting graceful shutdown...");
        }
    }

    Ok(())
}

fn init_tracing(log_level: &str) -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level)),
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

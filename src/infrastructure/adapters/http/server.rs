use crate::application::config::ServerConfig;
use crate::infrastructure::adapters::graphql::handlers::{graphql_handler, graphql_playground};
use crate::presentation::api::graphql::schema::AppSchema;
use crate::presentation::api::rest::{email_sender, health_check};
use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use actix_web_prometheus::PrometheusMetricsBuilder;
use anyhow::Context;
use std::sync::Arc;
use tracing::info;
use tracing_actix_web::TracingLogger;

pub async fn start(schema: Arc<AppSchema>, config: ServerConfig) -> anyhow::Result<()> {
    let bind_address = format!("{}:{}", config.host, config.port);
    info!("Booting HTTP server at http://{}", bind_address);

    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build Prometheus metrics: {}", e))?;

    let bind_address_clone = bind_address.clone();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .wrap(TracingLogger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(web::Data::from(schema.clone()))
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_handler))
                    .route(web::get().to(graphql_playground)),
            )
            .configure(health_check::configure)
            .configure(email_sender::configure)
    })
    .bind(&bind_address_clone)
    .with_context(|| format!("Failed to bind server to {}", bind_address_clone))?;

    info!(
        "✅ HTTP server successfully started on http://{}",
        bind_address
    );
    info!("🚀 GraphQL Playground: http://{}/graphql", bind_address);
    info!("📊 Prometheus Metrics: http://{}/metrics", bind_address);

    server
        .run()
        .await
        .map_err(|e| anyhow::anyhow!("Server runtime error: {}", e))
}

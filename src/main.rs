use rust_gateway::application::bootstrap;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    bootstrap::run().await
}

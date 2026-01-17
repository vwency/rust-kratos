mod application;
mod domain;
mod infrastructure;
use application::bootstrap;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    bootstrap::run().await
}

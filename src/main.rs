mod application;
mod domain;
mod infrastructure;
mod presentation;

use application::bootstrap;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    bootstrap::run().await
}

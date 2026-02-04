use rust_kratos::application::bootstrap::bootstrap;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    bootstrap::run().await
}

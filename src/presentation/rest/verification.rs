use crate::application::usecases::auth::verification::VerificationUseCase;
use crate::infrastructure::adapters::kratos::client::KratosClient;
use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct VerificationRequest {
    email: String,
}

#[post("/verification")]
async fn verification(
    body: web::Json<VerificationRequest>,
    kratos_client: web::Data<KratosClient>,
) -> impl Responder {
    match VerificationUseCase::execute(&body.email, &kratos_client, None).await {
        Ok(cookies) => HttpResponse::Ok().json(serde_json::json!({
            "cookies": cookies
        })),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e
        })),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(verification);
}

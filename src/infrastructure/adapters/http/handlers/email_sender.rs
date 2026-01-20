use actix_web::{HttpResponse, Responder, post};
use serde::Deserialize;
use tracing::instrument;

#[derive(Deserialize, Debug)]
struct EmailRequest {
    #[allow(dead_code)]
    to: Option<String>,
    #[allow(dead_code)]
    subject: Option<String>,
    #[allow(dead_code)]
    body: Option<String>,
}

#[post("/email")]
#[instrument]
async fn email(_payload: actix_web::web::Json<EmailRequest>) -> impl Responder {
    tracing::info!("Email endpoint called (stub)");
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Email stub - always returns 200"
    }))
}

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(email);
}

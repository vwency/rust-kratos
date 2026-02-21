use actix_web::{HttpResponse, Responder, get, web};
use serde::Deserialize;
use tracing::instrument;

#[derive(Deserialize, Debug)]
pub struct ErrorQuery {
    error: Option<String>,
    error_description: Option<String>,
}

#[get("/error")]
#[instrument]
pub async fn hydra_error(query: web::Query<ErrorQuery>) -> impl Responder {
    HttpResponse::BadRequest().json(serde_json::json!({
        "error": query.error,
        "error_description": query.error_description
    }))
}

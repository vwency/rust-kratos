use actix_web::{HttpResponse, Responder, get, web};
use serde::Deserialize;
use tracing::instrument;

#[derive(Deserialize, Debug)]
pub struct ChallengeQuery {
    logout_challenge: String,
}

#[get("/logout")]
#[instrument]
pub async fn logout(query: web::Query<ChallengeQuery>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "logout_challenge": query.logout_challenge
    }))
}

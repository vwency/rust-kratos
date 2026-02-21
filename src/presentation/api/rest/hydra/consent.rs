use actix_web::{HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Deserialize, Debug)]
pub struct ChallengeQuery {
    consent_challenge: String,
}

#[derive(Deserialize, Debug)]
pub struct ConsentForm {
    consent_challenge: String,
    grant_scope: Vec<String>,
    remember: Option<bool>,
}

#[derive(Serialize)]
struct AcceptConsentRequest {
    grant_scope: Vec<String>,
    remember: bool,
    remember_for: i64,
    session: ConsentSession,
}

#[derive(Serialize)]
struct ConsentSession {
    access_token: serde_json::Value,
    id_token: serde_json::Value,
}

#[get("/consent")]
#[instrument]
pub async fn consent_get(query: web::Query<ChallengeQuery>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "consent_challenge": query.consent_challenge
    }))
}

#[post("/consent")]
#[instrument]
pub async fn consent_post(form: web::Json<ConsentForm>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "consent_challenge": form.consent_challenge,
        "grant_scope": form.grant_scope
    }))
}

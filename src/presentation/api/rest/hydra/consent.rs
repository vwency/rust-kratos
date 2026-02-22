use crate::infrastructure::adapters::hydra::client::HydraClient;
use crate::infrastructure::adapters::hydra::requests::{accept_consent, get_consent_request};
use actix_web::{HttpResponse, Responder, get, post, web};
use serde::Deserialize;
use std::sync::Arc;
use tracing::instrument;

#[derive(Deserialize, Debug)]
pub struct ChallengeQuery {
    consent_challenge: String,
}

#[derive(Deserialize, Debug)]
pub struct ConsentForm {
    consent_challenge: String,
    grant_scope: Vec<String>,
}

#[get("/consent")]
#[instrument(skip(hydra))]
pub async fn consent_get(
    query: web::Query<ChallengeQuery>,
    hydra: web::Data<Arc<HydraClient>>,
) -> impl Responder {
    let consent_req = match get_consent_request(&hydra, &query.consent_challenge).await {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()}));
        }
    };

    let accepted = match accept_consent(
        &hydra,
        &query.consent_challenge,
        consent_req.requested_scope,
        3600,
    )
    .await
    {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()}));
        }
    };

    HttpResponse::Found()
        .insert_header(("Location", accepted.redirect_to))
        .finish()
}

#[post("/consent")]
#[instrument(skip(hydra))]
pub async fn consent_post(
    form: web::Json<ConsentForm>,
    hydra: web::Data<Arc<HydraClient>>,
) -> impl Responder {
    let consent_req = match get_consent_request(&hydra, &form.consent_challenge).await {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()}));
        }
    };

    let scope = if form.grant_scope.is_empty() {
        consent_req.requested_scope
    } else {
        form.grant_scope.clone()
    };

    let accepted = match accept_consent(&hydra, &form.consent_challenge, scope, 3600).await {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()}));
        }
    };

    HttpResponse::Found()
        .insert_header(("Location", accepted.redirect_to))
        .finish()
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(consent_get).service(consent_post);
}

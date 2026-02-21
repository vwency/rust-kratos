use crate::infrastructure::adapters::hydra::client::HydraClient;
use crate::infrastructure::adapters::hydra::requests::{accept_logout, get_logout_request};
use actix_web::{HttpResponse, Responder, get, web};
use serde::Deserialize;
use std::sync::Arc;
use tracing::instrument;

#[derive(Deserialize, Debug)]
pub struct ChallengeQuery {
    logout_challenge: String,
}

#[get("/logout")]
#[instrument(skip(hydra))]
pub async fn logout(
    query: web::Query<ChallengeQuery>,
    hydra: web::Data<Arc<HydraClient>>,
) -> impl Responder {
    match get_logout_request(&hydra, &query.logout_challenge).await {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()}));
        }
    };

    let accepted = match accept_logout(&hydra, &query.logout_challenge).await {
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

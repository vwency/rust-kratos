use crate::infrastructure::adapters::hydra::client::HydraClient;
use crate::infrastructure::adapters::hydra::requests::{accept_login, get_login_request};
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::http::flows::{fetch_flow, post_flow};
use actix_web::{HttpResponse, Responder, get, post, web};
use serde::Deserialize;
use std::sync::Arc;
use tracing::instrument;

#[derive(Deserialize, Debug)]
pub struct ChallengeQuery {
    login_challenge: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    email: String,
    password: String,
    login_challenge: String,
}

#[get("/login")]
#[instrument(skip(hydra))]
pub async fn login_get(
    query: web::Query<ChallengeQuery>,
    hydra: web::Data<Arc<HydraClient>>,
) -> impl Responder {
    let login_req = match get_login_request(&hydra, &query.login_challenge).await {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()}));
        }
    };

    if login_req.skip {
        let accepted =
            match accept_login(&hydra, &query.login_challenge, login_req.subject, 3600).await {
                Ok(r) => r,
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .json(serde_json::json!({"error": e.to_string()}));
                }
            };
        return HttpResponse::Found()
            .insert_header(("Location", accepted.redirect_to))
            .finish();
    }

    HttpResponse::Ok().json(serde_json::json!({
        "login_challenge": query.login_challenge
    }))
}

#[post("/login")]
#[instrument(skip(hydra, kratos))]
pub async fn login_post(
    form: web::Json<LoginForm>,
    hydra: web::Data<Arc<HydraClient>>,
    kratos: web::Data<Arc<KratosClient>>,
) -> impl Responder {
    let flow = match fetch_flow(&kratos.client, &kratos.public_url, "login", None).await {
        Ok(f) => f,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()}));
        }
    };

    let flow_id = match flow.flow["id"].as_str() {
        Some(id) => id.to_string(),
        None => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "flow id not found"}));
        }
    };

    let payload = serde_json::json!({
        "method": "password",
        "identifier": form.email,
        "password": form.password,
        "csrf_token": flow.csrf_token,
    });

    let result = match post_flow(
        &kratos.client,
        &kratos.public_url,
        "login",
        &flow_id,
        payload,
        &flow.cookies,
    )
    .await
    {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({"error": e.to_string()}));
        }
    };

    let subject = match result.data["session"]["identity"]["id"].as_str() {
        Some(id) => id.to_string(),
        None => {
            return HttpResponse::Unauthorized()
                .json(serde_json::json!({"error": "identity id not found"}));
        }
    };

    let accepted = match accept_login(&hydra, &form.login_challenge, subject, 3600).await {
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

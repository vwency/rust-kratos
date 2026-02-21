use actix_web::{HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};
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

#[derive(Serialize)]
struct AcceptLoginRequest {
    subject: String,
    remember: bool,
    remember_for: i64,
}

#[get("/login")]
#[instrument]
pub async fn login_get(query: web::Query<ChallengeQuery>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "login_challenge": query.login_challenge
    }))
}

#[post("/login")]
#[instrument]
pub async fn login_post(form: web::Json<LoginForm>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "login_challenge": form.login_challenge
    }))
}

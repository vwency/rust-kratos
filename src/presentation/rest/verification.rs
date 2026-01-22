use crate::application::usecases::auth::verification::VerificationUseCase;
use crate::infrastructure::adapters::kratos::client::KratosClient;
use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(untagged)]
enum VerificationRequest {
    Email {
        email: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        transient_payload: Option<Value>,
    },
    Code {
        code: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        transient_payload: Option<Value>,
    },
}

#[post("/verification")]
async fn verification(
    body: web::Json<VerificationRequest>,
    kratos_client: web::Data<KratosClient>,
) -> impl Responder {
    let result = match body.into_inner() {
        VerificationRequest::Email {
            email,
            transient_payload,
        } => {
            VerificationUseCase::execute_with_email(&email, &kratos_client, None, transient_payload)
                .await
        }
        VerificationRequest::Code {
            code,
            transient_payload,
        } => {
            VerificationUseCase::execute_with_code(&code, &kratos_client, None, transient_payload)
                .await
        }
    };

    match result {
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

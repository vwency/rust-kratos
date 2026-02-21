use crate::infrastructure::adapters::hydra::client::HydraClient;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub skip: bool,
    pub subject: String,
    pub redirect_to: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ConsentRequest {
    pub skip: bool,
    pub subject: String,
    pub requested_scope: Vec<String>,
    pub redirect_to: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct LogoutRequest {
    pub subject: String,
    pub redirect_to: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AcceptResponse {
    pub redirect_to: String,
}

#[derive(Serialize)]
struct AcceptLoginBody {
    subject: String,
    remember: bool,
    remember_for: i64,
}

#[derive(Serialize)]
struct AcceptConsentBody {
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

pub async fn get_login_request(
    hydra: &HydraClient,
    challenge: &str,
) -> Result<LoginRequest, reqwest::Error> {
    hydra
        .client
        .get(format!(
            "{}/admin/oauth2/auth/requests/login?login_challenge={}",
            hydra.admin_url, challenge
        ))
        .send()
        .await?
        .json::<LoginRequest>()
        .await
}

pub async fn accept_login(
    hydra: &HydraClient,
    challenge: &str,
    subject: String,
    remember_for: i64,
) -> Result<AcceptResponse, reqwest::Error> {
    hydra
        .client
        .put(format!(
            "{}/admin/oauth2/auth/requests/login/accept?login_challenge={}",
            hydra.admin_url, challenge
        ))
        .json(&AcceptLoginBody {
            subject,
            remember: true,
            remember_for,
        })
        .send()
        .await?
        .json::<AcceptResponse>()
        .await
}

pub async fn get_consent_request(
    hydra: &HydraClient,
    challenge: &str,
) -> Result<ConsentRequest, reqwest::Error> {
    hydra
        .client
        .get(format!(
            "{}/admin/oauth2/auth/requests/consent?consent_challenge={}",
            hydra.admin_url, challenge
        ))
        .send()
        .await?
        .json::<ConsentRequest>()
        .await
}

pub async fn accept_consent(
    hydra: &HydraClient,
    challenge: &str,
    grant_scope: Vec<String>,
    remember_for: i64,
) -> Result<AcceptResponse, reqwest::Error> {
    hydra
        .client
        .put(format!(
            "{}/admin/oauth2/auth/requests/consent/accept?consent_challenge={}",
            hydra.admin_url, challenge
        ))
        .json(&AcceptConsentBody {
            grant_scope,
            remember: true,
            remember_for,
            session: ConsentSession {
                access_token: serde_json::json!({}),
                id_token: serde_json::json!({}),
            },
        })
        .send()
        .await?
        .json::<AcceptResponse>()
        .await
}

pub async fn get_logout_request(
    hydra: &HydraClient,
    challenge: &str,
) -> Result<LogoutRequest, reqwest::Error> {
    hydra
        .client
        .get(format!(
            "{}/admin/oauth2/auth/requests/logout?logout_challenge={}",
            hydra.admin_url, challenge
        ))
        .send()
        .await?
        .json::<LogoutRequest>()
        .await
}

pub async fn accept_logout(
    hydra: &HydraClient,
    challenge: &str,
) -> Result<AcceptResponse, reqwest::Error> {
    hydra
        .client
        .put(format!(
            "{}/admin/oauth2/auth/requests/logout/accept?logout_challenge={}",
            hydra.admin_url, challenge
        ))
        .send()
        .await?
        .json::<AcceptResponse>()
        .await
}

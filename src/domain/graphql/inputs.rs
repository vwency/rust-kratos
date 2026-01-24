use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(InputObject, Clone)]
pub struct RegisterInput {
    pub email: String,
    pub username: String,
    pub password: String,
    pub geo_location: Option<String>,
}

#[derive(InputObject, Clone, Serialize, Deserialize, Debug)]
pub struct LoginInput {
    pub identifier: String,
    pub password: String,
    pub address: Option<String>,
    pub code: Option<String>,
    pub resend: Option<String>,
}

#[derive(InputObject, Serialize, Deserialize, Debug, Clone)]
pub struct RecoveryInput {
    pub email: String,
}

#[derive(InputObject)]
pub struct VerifyByLinkInput {
    pub email: String,
    pub transient_payload: Option<Value>,
}

#[derive(InputObject)]
pub struct SendVerificationCodeInput {
    pub email: String,
    pub transient_payload: Option<Value>,
}

#[derive(InputObject)]
pub struct SubmitVerificationCodeInput {
    pub code: String,
    pub transient_payload: Option<Value>,
}

#[derive(InputObject, Clone, Serialize, Deserialize, Debug)]
pub struct UpdateSettingsInput {
    pub method: String,
    pub password: Option<String>,
    pub traits: Option<Value>,
    pub lookup_secret_confirm: Option<bool>,
    pub lookup_secret_disable: Option<bool>,
    pub lookup_secret_regenerate: Option<bool>,
    pub lookup_secret_reveal: Option<bool>,
    pub transient_payload: Option<Value>,
}

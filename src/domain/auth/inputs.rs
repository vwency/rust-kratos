use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

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

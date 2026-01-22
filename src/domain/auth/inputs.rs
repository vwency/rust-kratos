use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(InputObject, Clone)]
pub struct RegisterInput {
    pub email: String,
    pub username: String,
    pub password: String,
    pub geo_location: Option<String>,
}

#[derive(InputObject, Clone)]
pub struct LoginInput {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: String,
    pub address: Option<String>,
    pub code: Option<String>,
    pub identifier: Option<String>,
    pub resend: Option<String>,
}

#[derive(InputObject, Serialize, Deserialize, Debug, Clone)]
pub struct RecoveryInput {
    pub email: String,
}

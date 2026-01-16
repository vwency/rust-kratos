use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

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
}

#[derive(InputObject, Serialize, Deserialize, Debug, Clone)]
pub struct RecoveryInput {
    pub email: String,
}

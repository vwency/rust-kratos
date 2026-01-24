use crate::domain::entities::user::UserProfile;
use crate::domain::ports::identity::{IdentityError, IdentityPort};
use crate::infrastructure::adapters::kratos::client::KratosClient;
use async_trait::async_trait;
use reqwest::header;
use std::sync::Arc;

#[allow(unused)]
pub struct KratosIdentityAdapter {
    client: Arc<KratosClient>,
}

#[allow(unused)]
impl KratosIdentityAdapter {
    pub fn new(client: Arc<KratosClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl IdentityPort for KratosIdentityAdapter {
    async fn get_current_user(&self, cookie: &str) -> Result<UserProfile, IdentityError> {
        let url = format!("{}/sessions/whoami", self.client.public_url);
        let url = url.replace("localhost", "127.0.0.1");
        let response = self
            .client
            .client
            .get(&url)
            .header(header::COOKIE, cookie)
            .send()
            .await
            .map_err(|e| IdentityError::NetworkError(e.to_string()))?;
        if !response.status().is_success() {
            return Err(IdentityError::NotAuthenticated);
        }
        let session_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| IdentityError::NetworkError(e.to_string()))?;
        let email = session_data["identity"]["traits"]["email"]
            .as_str()
            .ok_or_else(|| IdentityError::UnknownError("Email not found".to_string()))?
            .to_string();
        let username = session_data["identity"]["traits"]["username"]
            .as_str()
            .ok_or_else(|| IdentityError::UnknownError("Username not found".to_string()))?
            .to_string();
        let geo_location = session_data["identity"]["traits"]["geo_location"]
            .as_str()
            .map(|s| s.to_string());
        Ok(UserProfile {
            email,
            username,
            geo_location,
        })
    }
}

use crate::contexts::auth::domain::entities::user::UserProfile;
use crate::contexts::auth::domain::ports::session::{SessionError, SessionPort};
use crate::contexts::auth::infrastructure::adapters::kratos::client::KratosClient;
use async_trait::async_trait;
use reqwest::header;
use std::sync::Arc;

pub struct KratosSessionAdapter {
    client: Arc<KratosClient>,
}

impl KratosSessionAdapter {
    pub fn new(client: Arc<KratosClient>) -> Self {
        Self { client }
    }
    pub async fn get_current_user(&self, cookie: &str) -> Result<UserProfile, SessionError> {
        let url = format!("{}/sessions/whoami", self.client.public_url);
        let url = url.replace("localhost", "127.0.0.1");
        let response = self
            .client
            .client
            .get(&url)
            .header(header::COOKIE, cookie)
            .send()
            .await
            .map_err(|e| SessionError::NetworkError(e.to_string()))?;
        if !response.status().is_success() {
            return Err(SessionError::NotAuthenticated);
        }
        let session_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| SessionError::NetworkError(e.to_string()))?;
        let email = session_data["identity"]["traits"]["email"]
            .as_str()
            .ok_or_else(|| SessionError::UnknownError("Email not found".to_string()))?
            .to_string();
        let username = session_data["identity"]["traits"]["username"]
            .as_str()
            .ok_or_else(|| SessionError::UnknownError("Username not found".to_string()))?
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
    async fn get_logout_flow(&self, cookie: &str) -> Result<String, SessionError> {
        let url = format!("{}/self-service/logout/browser", self.client.public_url);
        let url = url.replace("localhost", "127.0.0.1");
        let response = self
            .client
            .client
            .get(&url)
            .header(header::COOKIE, cookie)
            .send()
            .await
            .map_err(|e| SessionError::NetworkError(e.to_string()))?;
        if !response.status().is_success() {
            return Err(SessionError::NetworkError(
                "Failed to get logout flow".to_string(),
            ));
        }
        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| SessionError::NetworkError(e.to_string()))?;
        data["logout_url"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| SessionError::UnknownError("Logout URL not found".to_string()))
    }
}

#[async_trait]
impl SessionPort for KratosSessionAdapter {
    async fn logout(&self, cookie: &str) -> Result<(), SessionError> {
        let logout_url = self.get_logout_flow(cookie).await?;
        let response = self
            .client
            .client
            .get(&logout_url)
            .header(header::COOKIE, cookie)
            .send()
            .await
            .map_err(|e| SessionError::NetworkError(e.to_string()))?;
        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(SessionError::UnknownError(format!(
                "Logout failed: {}",
                error_text
            )));
        }
        Ok(())
    }
    async fn check_active_session(&self, cookie: Option<&str>) -> bool {
        if let Some(cookie_value) = cookie {
            if let Ok(_) = self.get_current_user(cookie_value).await {
                return true;
            }
        }
        false
    }
}

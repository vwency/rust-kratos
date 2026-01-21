use crate::domain::kratos::models::{IdentityTraits, KratosIdentity, KratosSession};
use crate::infrastructure::adapters::kratos::client::KratosClient;
use reqwest::{StatusCode, header};

impl KratosClient {
    pub(crate) async fn check_active_session(&self, cookie: Option<&str>) -> bool {
        if let Some(cookie_value) = cookie {
            if let Ok(_) = self.handle_get_current_user(cookie_value).await {
                return true;
            }
        }
        false
    }

    pub async fn get_session(
        &self,
        cookie: &str,
    ) -> Result<Option<KratosSession>, Box<dyn std::error::Error>> {
        let url = format!("{}/sessions/whoami", self.public_url);
        let url = url.replace("localhost", "127.0.0.1");

        let response = self
            .client
            .get(&url)
            .header(header::COOKIE, cookie)
            .send()
            .await
            .map_err(|e| format!("Failed to connect to whoami endpoint: {}", e))?;

        let status = response.status();

        if status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN {
            return Ok(None);
        }

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!(
                "Failed to fetch session (status {}): {}",
                status, error_text
            )
            .into());
        }

        let session_json: serde_json::Value = response.json().await?;

        let session = KratosSession {
            id: session_json["id"].as_str().unwrap_or_default().to_string(),
            active: session_json["active"].as_bool().unwrap_or(false),
            identity: KratosIdentity {
                id: session_json["identity"]["id"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                schema_id: session_json["identity"]["schema_id"]
                    .as_str()
                    .unwrap_or("default")
                    .to_string(),
                traits: IdentityTraits {
                    email: session_json["identity"]["traits"]["email"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    username: session_json["identity"]["traits"]["username"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    geo_location: session_json["identity"]["traits"]["geo_location"]
                        .as_str()
                        .map(|s| s.to_string()),
                },
                created_at: session_json["identity"]["created_at"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                updated_at: session_json["identity"]["updated_at"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
            },
        };

        Ok(Some(session))
    }
}

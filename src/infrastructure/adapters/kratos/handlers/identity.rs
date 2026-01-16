use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::models::{IdentityTraits, KratosIdentity};
use reqwest::header;

impl KratosClient {
    pub(crate) fn parse_identity(
        data: &serde_json::Value,
    ) -> Result<KratosIdentity, Box<dyn std::error::Error>> {
        let identity_data = data
            .get("identity")
            .ok_or("Identity not found in response")?;

        Ok(KratosIdentity {
            id: identity_data["id"]
                .as_str()
                .ok_or("Identity ID not found")?
                .to_string(),
            schema_id: identity_data["schema_id"]
                .as_str()
                .unwrap_or("default")
                .to_string(),
            traits: IdentityTraits {
                email: identity_data["traits"]["email"]
                    .as_str()
                    .ok_or("Email not found")?
                    .to_string(),
                username: identity_data["traits"]["username"]
                    .as_str()
                    .ok_or("Username not found")?
                    .to_string(),
                geo_location: identity_data["traits"]["geo_location"]
                    .as_str()
                    .map(|s| s.to_string()),
            },
            created_at: identity_data["created_at"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            updated_at: identity_data["updated_at"]
                .as_str()
                .unwrap_or("")
                .to_string(),
        })
    }

    pub async fn handle_get_current_user(
        &self,
        cookie: &str,
    ) -> Result<IdentityTraits, Box<dyn std::error::Error>> {
        let url = format!("{}/sessions/whoami", self.public_url);
        let url = url.replace("localhost", "127.0.0.1");

        let response = self
            .client
            .get(&url)
            .header(header::COOKIE, cookie)
            .send()
            .await
            .map_err(|e| format!("Failed to connect to whoami endpoint: {}", e))?;

        if !response.status().is_success() {
            return Err("Not logged in".into());
        }

        let session_data: serde_json::Value = response.json().await?;

        let traits = IdentityTraits {
            email: session_data["identity"]["traits"]["email"]
                .as_str()
                .ok_or("Email not found")?
                .to_string(),
            username: session_data["identity"]["traits"]["username"]
                .as_str()
                .ok_or("Username not found")?
                .to_string(),
            geo_location: session_data["identity"]["traits"]["geo_location"]
                .as_str()
                .map(|s| s.to_string()),
        };

        Ok(traits)
    }
}

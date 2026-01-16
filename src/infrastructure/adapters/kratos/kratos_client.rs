use crate::infrastructure::adapters::kratos::flows::{
    FlowResult, PostFlowResult, fetch_flow, post_flow,
};
use reqwest::StatusCode;
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone)]
pub struct KratosClient {
    client: Client,
    admin_url: String,
    public_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KratosIdentity {
    pub id: String,
    pub schema_id: String,
    pub traits: IdentityTraits,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityTraits {
    pub email: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KratosSession {
    pub id: String,
    pub active: bool,
    pub identity: KratosIdentity,
}

impl KratosClient {
    pub fn new(admin_url: String, public_url: String) -> Self {
        let client = Client::builder()
            .cookie_store(false)
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(120))
            .connect_timeout(Duration::from_secs(30))
            .pool_idle_timeout(Duration::from_secs(120))
            .pool_max_idle_per_host(10)
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            admin_url,
            public_url,
        }
    }

    async fn check_active_session(&self, cookie: Option<&str>) -> bool {
        if let Some(cookie_value) = cookie {
            if let Ok(_) = self.handle_get_current_user(cookie_value).await {
                return true;
            }
        }
        false
    }

    fn parse_identity(
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

    pub async fn handle_signup(
        &self,
        email: &str,
        username: &str,
        password: &str,
        cookie: Option<&str>,
    ) -> Result<(KratosSession, Vec<String>), Box<dyn std::error::Error>> {
        if self.check_active_session(cookie).await {
            return Err(Box::from(
                "Already logged in. Please logout first before registering.",
            ));
        }

        let flow_result =
            fetch_flow(&self.client, &self.public_url, "registration", cookie).await?;

        let registration_data = serde_json::json!({
            "method": "password",
            "password": password,
            "traits": {
                "email": email,
                "username": username
            },
            "csrf_token": flow_result.csrf_token,
        });

        let post_result = post_flow(
            &self.client,
            &self.public_url,
            "registration",
            flow_result.flow["id"].as_str().ok_or("Flow ID not found")?,
            registration_data,
            &flow_result.cookies,
        )
        .await?;

        let response_data = &post_result.data;

        let (identity, session_id) = if let Some(session_data) = response_data.get("session") {
            let identity = Self::parse_identity(session_data)?;
            let session_id = session_data["id"]
                .as_str()
                .ok_or("Session ID not found")?
                .to_string();
            (identity, session_id)
        } else if response_data.get("identity").is_some() {
            let identity = Self::parse_identity(response_data)?;
            if post_result
                .cookies
                .iter()
                .any(|c| c.contains("ory_kratos_session"))
            {
                (identity, "session_from_cookie".to_string())
            } else {
                return Err("Registration succeeded but no session was created".into());
            }
        } else {
            return Err(
                "Registration failed: neither session nor identity found in response".into(),
            );
        };

        let session = KratosSession {
            id: session_id,
            active: true,
            identity,
        };

        Ok((session, post_result.cookies))
    }

    pub async fn handle_login(
        &self,
        identifier: &str,
        password: &str,
        cookie: Option<&str>,
    ) -> Result<(KratosSession, Vec<String>), Box<dyn std::error::Error>> {
        if self.check_active_session(cookie).await {
            return Err(Box::from(
                "Already logged in. Please logout first before logging in again.",
            ));
        }

        let flow_result = fetch_flow(&self.client, &self.public_url, "login", cookie).await?;

        let login_data = serde_json::json!({
            "method": "password",
            "password": password,
            "identifier": identifier,
            "csrf_token": flow_result.csrf_token,
        });

        let post_result = post_flow(
            &self.client,
            &self.public_url,
            "login",
            flow_result.flow["id"].as_str().ok_or("Flow ID not found")?,
            login_data,
            &flow_result.cookies,
        )
        .await?;

        let response_data = &post_result.data;

        let (identity, session_id) = if let Some(session_data) = response_data.get("session") {
            let identity = Self::parse_identity(session_data)?;
            let session_id = session_data["id"]
                .as_str()
                .ok_or("Session ID not found")?
                .to_string();
            (identity, session_id)
        } else if response_data.get("identity").is_some() {
            let identity = Self::parse_identity(response_data)?;
            if post_result
                .cookies
                .iter()
                .any(|c| c.contains("ory_kratos_session"))
            {
                (identity, "session_from_cookie".to_string())
            } else {
                return Err("Login succeeded but no session was created".into());
            }
        } else {
            return Err("Invalid response format: neither 'session' nor 'identity' found".into());
        };

        let session = KratosSession {
            id: session_id,
            active: true,
            identity,
        };

        Ok((session, post_result.cookies))
    }

    pub async fn handle_logout(
        &self,
        cookie: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!("{}/self-service/logout/browser", self.public_url);
        let url = url.replace("localhost", "127.0.0.1");

        let flow_response = self
            .client
            .get(&url)
            .header(header::COOKIE, cookie)
            .send()
            .await
            .map_err(|e| format!("Failed to connect to logout endpoint: {}", e))?;

        if !flow_response.status().is_success() {
            let error_text = flow_response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Failed to get logout flow: {}", error_text).into());
        }

        let flow_data: serde_json::Value = flow_response.json().await?;
        let logout_url = flow_data["logout_url"]
            .as_str()
            .ok_or("Logout URL not found")?
            .replace("localhost", "127.0.0.1");

        let response = self
            .client
            .get(&logout_url)
            .header(header::COOKIE, cookie)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Logout failed: {}", error_text).into());
        }

        let cookies: Vec<String> = response
            .headers()
            .get_all(header::SET_COOKIE)
            .iter()
            .filter_map(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .collect();

        Ok(cookies)
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

use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::flows::{fetch_flow, post_flow};
use crate::infrastructure::adapters::kratos::models::{KratosIdentity, KratosSession};
use reqwest::header;

impl KratosClient {
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
}

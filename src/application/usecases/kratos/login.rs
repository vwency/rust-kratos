use crate::domain::kratos::flows::FlowResult;
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::flows::{fetch_flow, post_flow};
use tracing::error;

impl KratosClient {
    pub async fn get_login_flow(
        &self,
        cookie: Option<&str>,
    ) -> Result<FlowResult, Box<dyn std::error::Error>> {
        if self.check_active_session(cookie).await {
            error!("Login attempt with an already active session");
            return Err("Already logged in. Please log out first.".into());
        }

        fetch_flow(&self.client, &self.public_url, "login", cookie).await
    }

    pub async fn submit_login_flow(
        &self,
        flow_id: &str,
        csrf_token: &str,
        identifier: &str,
        password: &str,
        address: Option<&str>,
        code: Option<&str>,
        resend: Option<&str>,
        flow_cookies: Vec<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut payload = serde_json::json!({
            "method": "password",
            "identifier": identifier,
            "password": password,
            "csrf_token": csrf_token,
        });

        if let Some(addr) = address {
            payload["address"] = serde_json::json!(addr);
        }

        if let Some(c) = code {
            payload["code"] = serde_json::json!(c);
            payload["method"] = serde_json::json!("code");
        }

        if let Some(r) = resend {
            payload["resend"] = serde_json::json!(r);
        }

        let result = post_flow(
            &self.client,
            &self.public_url,
            "login",
            flow_id,
            payload,
            &flow_cookies,
        )
        .await?;

        Ok(result.cookies)
    }
}

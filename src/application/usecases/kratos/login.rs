use crate::domain::kratos::flows::FlowResult;
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::flows::{fetch_flow, post_flow};

impl KratosClient {
    pub async fn get_login_flow(
        &self,
        cookie: Option<&str>,
    ) -> Result<FlowResult, Box<dyn std::error::Error>> {
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
        flow_cookies: &[String],
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut login_data = serde_json::json!({
            "method": "password",
            "identifier": identifier,
            "password": password,
            "csrf_token": csrf_token,
        });

        if let Some(addr) = address {
            login_data["address"] = serde_json::json!(addr);
        }

        if let Some(c) = code {
            login_data["code"] = serde_json::json!(c);
            login_data["method"] = serde_json::json!("code");
        }

        if let Some(r) = resend {
            login_data["resend"] = serde_json::json!(r);
        }

        let post_result = post_flow(
            &self.client,
            &self.public_url,
            "login",
            flow_id,
            login_data,
            flow_cookies,
        )
        .await?;

        Ok(post_result.cookies)
    }
}

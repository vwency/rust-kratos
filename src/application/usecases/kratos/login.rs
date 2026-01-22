use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::flows::{fetch_flow, post_flow};

impl KratosClient {
    pub async fn login(
        &self,
        identifier: &str,
        password: &str,
        address: Option<&str>,
        code: Option<&str>,
        code_identifier: Option<&str>,
        resend: Option<&str>,
        cookie: Option<&str>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let flow_result = fetch_flow(&self.client, &self.public_url, "login", cookie).await?;

        let mut login_data = serde_json::json!({
            "method": "password",
            "identifier": identifier,
            "password": password,
            "csrf_token": flow_result.csrf_token,
        });

        if let Some(addr) = address {
            login_data["address"] = serde_json::json!(addr);
        }

        if let Some(c) = code {
            login_data["code"] = serde_json::json!(c);
            login_data["method"] = serde_json::json!("code");
        }

        if let Some(id) = code_identifier {
            login_data["identifier"] = serde_json::json!(id);
        }

        if let Some(r) = resend {
            login_data["resend"] = serde_json::json!(r);
        }

        let post_result = post_flow(
            &self.client,
            &self.public_url,
            "login",
            flow_result.flow["id"].as_str().ok_or("Flow ID not found")?,
            login_data,
            &flow_result.cookies,
        )
        .await?;

        Ok(post_result.cookies)
    }
}

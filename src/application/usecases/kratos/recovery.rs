use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::flows::{fetch_flow, post_flow};

impl KratosClient {
    pub async fn get_recovery_flow(
        &self,
        cookie: Option<&str>,
    ) -> Result<(String, String, Vec<String>), Box<dyn std::error::Error>> {
        let flow_result = fetch_flow(&self.client, &self.public_url, "recovery", cookie).await?;
        let flow_id = flow_result.flow["id"]
            .as_str()
            .ok_or("Flow ID not found")?
            .to_string();
        Ok((flow_id, flow_result.csrf_token, flow_result.cookies))
    }

    pub async fn recovery(
        &self,
        email: &str,
        cookie: Option<&str>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let (flow_id, csrf_token, cookies) = self.get_recovery_flow(cookie).await?;
        let recovery_data = serde_json::json!({
            "method": "link",
            "email": email,
            "csrf_token": csrf_token,
        });
        let post_result = post_flow(
            &self.client,
            &self.public_url,
            "recovery",
            &flow_id,
            recovery_data,
            &cookies,
        )
        .await?;
        Ok(post_result.cookies)
    }
}

use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::flows::{fetch_flow, post_flow};

impl KratosClient {
    pub async fn verification(
        &self,
        email: &str,
        cookie: Option<&str>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let flow_result =
            fetch_flow(&self.client, &self.public_url, "verification", cookie).await?;

        let verification_data = serde_json::json!({
            "method": "link",
            "email": email,
            "csrf_token": flow_result.csrf_token,
        });

        let post_result = post_flow(
            &self.client,
            &self.public_url,
            "verification",
            flow_result.flow["id"].as_str().ok_or("Flow ID not found")?,
            verification_data,
            &flow_result.cookies,
        )
        .await?;

        Ok(post_result.cookies)
    }
}

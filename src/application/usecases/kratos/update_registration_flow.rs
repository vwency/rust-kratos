use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::flows::{fetch_flow, post_flow};

impl KratosClient {
    pub async fn update_register_flow(
        &self,
        email: &str,
        username: &str,
        password: &str,
        cookie: Option<&str>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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

        if response_data.get("session").is_none() && response_data.get("identity").is_none() {
            return Err(
                "Registration failed: neither session nor identity found in response".into(),
            );
        }

        if !post_result
            .cookies
            .iter()
            .any(|c| c.contains("ory_kratos_session"))
        {
            return Err("Registration succeeded but no session cookie was created".into());
        }

        Ok(post_result.cookies)
    }
}

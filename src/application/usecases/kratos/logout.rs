use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::flows::fetch_flow;
use reqwest::header;

impl KratosClient {
    async fn get_logout_flow(&self, cookie: &str) -> Result<String, Box<dyn std::error::Error>> {
        let flow_result =
            fetch_flow(&self.client, &self.public_url, "logout", Some(cookie)).await?;
        let logout_url = flow_result.flow["logout_url"]
            .as_str()
            .ok_or("Logout URL not found")?
            .replace("localhost", "127.0.0.1");
        Ok(logout_url)
    }

    pub async fn logout(&self, cookie: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let logout_url = self.get_logout_flow(cookie).await?;
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

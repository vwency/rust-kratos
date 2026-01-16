use crate::infrastructure::adapters::kratos::client::KratosClient;
use reqwest::header;

impl KratosClient {
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

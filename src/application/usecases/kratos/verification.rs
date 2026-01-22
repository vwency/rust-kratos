use crate::domain::kratos::flows::FlowResult;
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::flows::{fetch_flow, post_flow};
use serde_json::Value;

impl KratosClient {
    pub async fn get_verification_flow(
        &self,
        cookie: Option<&str>,
    ) -> Result<FlowResult, Box<dyn std::error::Error>> {
        fetch_flow(&self.client, &self.public_url, "verification", cookie).await
    }

    pub async fn verification_link(
        &self,
        email: &str,
        flow_result: &FlowResult,
        transient_payload: Option<Value>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut verification_data = serde_json::json!({
            "method": "link",
            "email": email,
            "csrf_token": flow_result.csrf_token,
        });

        if let Some(payload) = transient_payload {
            verification_data["transient_payload"] = payload;
        }

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

    pub async fn verification_code_send(
        &self,
        email: &str,
        flow_result: &FlowResult,
        transient_payload: Option<Value>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut verification_data = serde_json::json!({
            "method": "code",
            "email": email,
            "csrf_token": flow_result.csrf_token,
        });

        if let Some(payload) = transient_payload {
            verification_data["transient_payload"] = payload;
        }

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

    pub async fn verification_code_submit(
        &self,
        code: &str,
        flow_result: &FlowResult,
        transient_payload: Option<Value>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut verification_data = serde_json::json!({
            "method": "code",
            "code": code,
            "csrf_token": flow_result.csrf_token,
        });

        if let Some(payload) = transient_payload {
            verification_data["transient_payload"] = payload;
        }

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

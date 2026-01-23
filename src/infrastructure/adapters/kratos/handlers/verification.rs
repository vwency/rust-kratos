use crate::domain::ports::verification::{
    SendCodeRequest, SubmitCodeRequest, VerificationError, VerificationPort, VerifyByLinkRequest,
};
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::flows::{fetch_flow, post_flow};
use async_trait::async_trait;

#[allow(unused)]
pub struct KratosVerificationAdapter {
    client: KratosClient,
}

#[allow(unused)]
impl KratosVerificationAdapter {
    pub fn new(client: KratosClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl VerificationPort for KratosVerificationAdapter {
    async fn verify_by_link(
        &self,
        request: VerifyByLinkRequest,
        cookie: Option<&str>,
    ) -> Result<(), VerificationError> {
        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "verification",
            cookie,
        )
        .await
        .map_err(|e| VerificationError::NetworkError(e.to_string()))?;

        let flow_id = flow.flow["id"]
            .as_str()
            .ok_or(VerificationError::FlowNotFound)?;

        let mut payload = serde_json::json!({
            "method": "link",
            "email": request.email,
            "csrf_token": flow.csrf_token,
        });

        if let Some(transient) = request.transient_payload {
            payload["transient_payload"] = transient;
        }

        post_flow(
            &self.client.client,
            &self.client.public_url,
            "verification",
            flow_id,
            payload,
            &flow.cookies,
        )
        .await
        .map_err(|e| VerificationError::NetworkError(e.to_string()))?;

        Ok(())
    }

    async fn send_verification_code(
        &self,
        request: SendCodeRequest,
        cookie: Option<&str>,
    ) -> Result<(), VerificationError> {
        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "verification",
            cookie,
        )
        .await
        .map_err(|e| VerificationError::NetworkError(e.to_string()))?;

        let flow_id = flow.flow["id"]
            .as_str()
            .ok_or(VerificationError::FlowNotFound)?;

        let mut payload = serde_json::json!({
            "method": "code",
            "email": request.email,
            "csrf_token": flow.csrf_token,
        });

        if let Some(transient) = request.transient_payload {
            payload["transient_payload"] = transient;
        }

        post_flow(
            &self.client.client,
            &self.client.public_url,
            "verification",
            flow_id,
            payload,
            &flow.cookies,
        )
        .await
        .map_err(|e| VerificationError::NetworkError(e.to_string()))?;

        Ok(())
    }

    async fn submit_verification_code(
        &self,
        request: SubmitCodeRequest,
        cookie: &str,
    ) -> Result<(), VerificationError> {
        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "verification",
            Some(cookie),
        )
        .await
        .map_err(|e| VerificationError::NetworkError(e.to_string()))?;

        let flow_id = flow.flow["id"]
            .as_str()
            .ok_or(VerificationError::FlowNotFound)?;

        let mut payload = serde_json::json!({
            "method": "code",
            "code": request.code,
            "csrf_token": flow.csrf_token,
        });

        if let Some(transient) = request.transient_payload {
            payload["transient_payload"] = transient;
        }

        post_flow(
            &self.client.client,
            &self.client.public_url,
            "verification",
            flow_id,
            payload,
            &flow.cookies,
        )
        .await
        .map_err(|e| VerificationError::NetworkError(e.to_string()))?;

        Ok(())
    }
}

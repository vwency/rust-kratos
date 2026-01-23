use crate::domain::ports::recovery::{RecoveryError, RecoveryPort, RecoveryRequest};
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::flows::{fetch_flow, post_flow};
use async_trait::async_trait;
use tracing::debug;

#[allow(unused)]
pub struct KratosRecoveryAdapter {
    client: KratosClient,
}

#[allow(unused)]
impl KratosRecoveryAdapter {
    pub fn new(client: KratosClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl RecoveryPort for KratosRecoveryAdapter {
    async fn initiate_recovery(
        &self,
        request: RecoveryRequest,
        cookie: Option<&str>,
    ) -> Result<(), RecoveryError> {
        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "recovery",
            cookie,
        )
        .await
        .map_err(|e| RecoveryError::NetworkError(e.to_string()))?;

        let flow_id = flow.flow["id"]
            .as_str()
            .ok_or(RecoveryError::FlowNotFound)?;

        let payload = serde_json::json!({
            "method": "link",
            "email": request.email,
            "csrf_token": flow.csrf_token,
        });

        let result = post_flow(
            &self.client.client,
            &self.client.public_url,
            "recovery",
            flow_id,
            payload,
            &flow.cookies,
        )
        .await
        .map_err(|e| RecoveryError::NetworkError(e.to_string()))?;

        if result.cookies.is_empty() {
            debug!("No cookies returned from Kratos");
        } else {
            debug!(
                cookies_count = result.cookies.len(),
                cookies = ?result.cookies,
                "Cookies returned from Kratos"
            );
        }

        Ok(())
    }
}

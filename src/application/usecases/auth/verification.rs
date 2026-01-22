use crate::infrastructure::adapters::kratos::client::KratosClient;
use serde_json::Value;

pub struct VerificationUseCase;

impl VerificationUseCase {
    pub async fn execute_link(
        email: &str,
        kratos_client: &KratosClient,
        cookie: Option<&str>,
        transient_payload: Option<Value>,
    ) -> Result<Vec<String>, String> {
        let flow_result = kratos_client
            .get_verification_flow(cookie)
            .await
            .map_err(|e| format!("Failed to get verification flow: {}", e))?;

        kratos_client
            .verification_link(email, &flow_result, transient_payload)
            .await
            .map_err(|e| format!("Verification link failed: {}", e))
    }

    pub async fn execute_code_send(
        email: &str,
        kratos_client: &KratosClient,
        cookie: Option<&str>,
        transient_payload: Option<Value>,
    ) -> Result<Vec<String>, String> {
        let flow_result = kratos_client
            .get_verification_flow(cookie)
            .await
            .map_err(|e| format!("Failed to get verification flow: {}", e))?;

        kratos_client
            .verification_code_send(email, &flow_result, transient_payload)
            .await
            .map_err(|e| format!("Failed to send verification code: {}", e))
    }

    pub async fn execute_code_submit(
        code: &str,
        kratos_client: &KratosClient,
        cookie: &str,
        transient_payload: Option<Value>,
    ) -> Result<Vec<String>, String> {
        let flow_result = kratos_client
            .get_verification_flow(Some(cookie))
            .await
            .map_err(|e| format!("Failed to get verification flow: {}", e))?;

        kratos_client
            .verification_code_submit(code, &flow_result, transient_payload)
            .await
            .map_err(|e| format!("Failed to submit verification code: {}", e))
    }
}

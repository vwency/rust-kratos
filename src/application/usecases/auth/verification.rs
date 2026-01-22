use crate::infrastructure::adapters::kratos::client::KratosClient;
use serde_json::Value;

pub struct VerificationUseCase;

impl VerificationUseCase {
    pub async fn execute_with_email(
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
            .verification(email, &flow_result, transient_payload)
            .await
            .map_err(|e| format!("Verification failed: {}", e))
    }

    pub async fn execute_with_code(
        code: &str,
        kratos_client: &KratosClient,
        cookie: Option<&str>,
        transient_payload: Option<Value>,
    ) -> Result<Vec<String>, String> {
        let flow_result = kratos_client
            .get_verification_flow(cookie)
            .await
            .map_err(|e| format!("Failed to get verification flow: {}", e))?;

        kratos_client
            .verification_with_code(code, &flow_result, transient_payload)
            .await
            .map_err(|e| format!("Verification failed: {}", e))
    }
}

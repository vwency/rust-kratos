use crate::infrastructure::adapters::kratos::client::KratosClient;

pub struct VerificationUseCase;

impl VerificationUseCase {
    pub async fn execute(
        email: &str,
        kratos_client: &KratosClient,
        cookie: Option<&str>,
    ) -> Result<Vec<String>, String> {
        match kratos_client.verification(email, cookie).await {
            Ok(cookies) => Ok(cookies),
            Err(e) => Err(format!("Verification failed: {}", e)),
        }
    }
}

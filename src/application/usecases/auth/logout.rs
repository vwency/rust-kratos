use crate::infrastructure::adapters::kratos::KratosClient;

pub struct LogoutUseCase;

impl LogoutUseCase {
    pub async fn execute(
        kratos_client: &KratosClient,
        cookie: Option<&str>,
    ) -> Result<Vec<String>, String> {
        kratos_client
            .handle_logout(cookie.unwrap_or(""))
            .await
            .map_err(|e| format!("Logout failed: {}", e))
    }
}

use crate::infrastructure::adapters::kratos::{KratosClient, models::IdentityTraits};

pub struct GetCurrentUserUseCase;

impl GetCurrentUserUseCase {
    pub async fn execute(
        kratos_client: &KratosClient,
        cookie: Option<&str>,
    ) -> Result<IdentityTraits, String> {
        let traits: IdentityTraits = kratos_client
            .handle_get_current_user(cookie.unwrap_or(""))
            .await
            .map_err(|e| format!("Failed to get current user: {}", e))?;

        Ok(traits)
    }
}

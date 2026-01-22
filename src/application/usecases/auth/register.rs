use crate::domain::auth::inputs::RegisterInput;
use crate::infrastructure::adapters::kratos::KratosClient;

pub struct RegisterUseCase;

impl RegisterUseCase {
    pub async fn execute(
        input: RegisterInput,
        kratos_client: &KratosClient,
        cookie: Option<&str>,
    ) -> Result<Vec<String>, String> {
        let cookies = kratos_client
            .update_register_flow(&input.email, &input.username, &input.password, cookie)
            .await
            .map_err(|e| format!("Failed to register: {}", e))?;

        Ok(cookies)
    }
}

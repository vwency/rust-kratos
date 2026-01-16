use crate::domain::auth::inputs::RegisterInput;
use crate::domain::auth::responses::AuthResponse;
use crate::infrastructure::adapters::kratos::KratosClient;

pub struct RegisterUseCase;

impl RegisterUseCase {
    pub async fn execute(
        input: RegisterInput,
        kratos_client: &KratosClient,
        cookie: Option<&str>,
    ) -> Result<(AuthResponse, Vec<String>), String> {
        let (session, cookies) = kratos_client
            .handle_signup(&input.email, &input.username, &input.password, cookie)
            .await
            .map_err(|e| format!("Failed to register: {}", e))?;

        Ok((
            AuthResponse::from_kratos_identity(session.identity, String::new()),
            cookies,
        ))
    }
}

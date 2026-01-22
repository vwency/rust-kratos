use crate::domain::auth::inputs::RegisterInput;
use crate::infrastructure::adapters::kratos::KratosClient;

pub struct RegisterUseCase;

impl RegisterUseCase {
    pub async fn execute(
        input: RegisterInput,
        kratos_client: &KratosClient,
        cookie: Option<&str>,
    ) -> Result<Vec<String>, String> {
        let (flow_id, csrf_token, flow_cookies) = kratos_client
            .get_registration_flow(cookie)
            .await
            .map_err(|e| format!("Failed to get registration flow: {}", e))?;

        let cookies = kratos_client
            .update_registration_flow(
                &flow_id,
                &csrf_token,
                &input.email,
                &input.username,
                &input.password,
                flow_cookies,
            )
            .await
            .map_err(|e| format!("Failed to complete registration: {}", e))?;

        Ok(cookies)
    }
}

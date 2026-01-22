use crate::domain::auth::inputs::LoginInput;
use crate::infrastructure::adapters::kratos::KratosClient;

pub struct LoginUseCase;

impl LoginUseCase {
    pub async fn execute(
        input: LoginInput,
        kratos_client: &KratosClient,
        cookie: Option<&str>,
    ) -> Result<Vec<String>, String> {
        let flow = kratos_client
            .get_login_flow(cookie)
            .await
            .map_err(|e| format!("Failed to get login flow: {}", e))?;

        let flow_id = flow.flow["id"]
            .as_str()
            .ok_or("Flow ID not found")?
            .to_string();

        let cookies = kratos_client
            .submit_login_flow(
                &flow_id,
                &flow.csrf_token,
                &input.identifier,
                &input.password,
                input.address.as_deref(),
                input.code.as_deref(),
                input.resend.as_deref(),
                flow.cookies,
            )
            .await
            .map_err(|e| format!("Login failed: {}", e))?;

        Ok(cookies)
    }
}

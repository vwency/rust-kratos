use crate::domain::graphql::inputs::LoginInput;
use crate::domain::ports::auth::AuthenticationPort;

pub struct LoginUseCase {
    auth_port: Box<dyn AuthenticationPort>,
}

#[allow(unused)]
impl LoginUseCase {
    pub fn new(auth_port: Box<dyn AuthenticationPort>) -> Self {
        Self { auth_port }
    }

    pub async fn execute(&self, input: LoginInput, cookie: Option<&str>) -> Result<String, String> {
        let flow_id = self
            .auth_port
            .initiate_login(cookie)
            .await
            .map_err(|e| e.to_string())?;

        let session_token = self
            .auth_port
            .complete_login(&flow_id, input.into())
            .await
            .map_err(|e| e.to_string())?;

        Ok(session_token)
    }
}

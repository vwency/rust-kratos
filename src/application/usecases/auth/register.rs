use crate::domain::graphql::inputs::RegisterInput;
use crate::domain::ports::{RegistrationError, RegistrationPort};

pub struct RegisterUseCase {
    registration_port: Box<dyn RegistrationPort>,
}

#[allow(unused)]
impl RegisterUseCase {
    pub fn new(registration_port: Box<dyn RegistrationPort>) -> Self {
        Self { registration_port }
    }

    pub async fn execute(
        &self,
        input: RegisterInput,
        cookie: Option<&str>,
    ) -> Result<String, RegistrationError> {
        let flow_id = self.registration_port.initiate_registration(cookie).await?;

        let session_token = self
            .registration_port
            .complete_registration(&flow_id, input.into())
            .await?;

        Ok(session_token)
    }
}

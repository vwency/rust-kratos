use crate::domain::graphql::inputs::{
    SendVerificationCodeInput, SubmitVerificationCodeInput, VerifyByLinkInput,
};
use crate::domain::ports::{VerificationError, VerificationPort};

pub struct VerificationUseCase {
    verification_port: Box<dyn VerificationPort>,
}

#[allow(unused)]
impl VerificationUseCase {
    pub fn new(verification_port: Box<dyn VerificationPort>) -> Self {
        Self { verification_port }
    }

    pub async fn execute_link(
        &self,
        input: VerifyByLinkInput,
        cookie: Option<&str>,
    ) -> Result<(), VerificationError> {
        self.verification_port
            .verify_by_link(input.into(), cookie)
            .await
    }

    pub async fn execute_code_send(
        &self,
        input: SendVerificationCodeInput,
        cookie: Option<&str>,
    ) -> Result<(), VerificationError> {
        self.verification_port
            .send_verification_code(input.into(), cookie)
            .await
    }

    pub async fn execute_code_submit(
        &self,
        input: SubmitVerificationCodeInput,
        cookie: &str,
    ) -> Result<(), VerificationError> {
        self.verification_port
            .submit_verification_code(input.into(), cookie)
            .await
    }
}

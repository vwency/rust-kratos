use crate::domain::graphql::inputs::{
    SendVerificationCodeInput, SubmitVerificationCodeInput, VerifyByLinkInput,
};
use async_trait::async_trait;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct VerifyByLinkRequest {
    pub email: String,
    pub transient_payload: Option<Value>,
}

impl From<VerifyByLinkInput> for VerifyByLinkRequest {
    fn from(input: VerifyByLinkInput) -> Self {
        Self {
            email: input.email,
            transient_payload: input.transient_payload,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SendCodeRequest {
    pub email: String,
    pub transient_payload: Option<Value>,
}

impl From<SendVerificationCodeInput> for SendCodeRequest {
    fn from(input: SendVerificationCodeInput) -> Self {
        Self {
            email: input.email,
            transient_payload: input.transient_payload,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubmitCodeRequest {
    pub code: String,
    pub transient_payload: Option<Value>,
}

impl From<SubmitVerificationCodeInput> for SubmitCodeRequest {
    fn from(input: SubmitVerificationCodeInput) -> Self {
        Self {
            code: input.code,
            transient_payload: input.transient_payload,
        }
    }
}

#[derive(Debug)]
pub enum VerificationError {
    FlowNotFound,
    InvalidCode,
    InvalidEmail,
    NetworkError(String),
    UnknownError(String),
}

impl std::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationError::FlowNotFound => write!(f, "Verification flow not found"),
            VerificationError::InvalidCode => write!(f, "Invalid verification code"),
            VerificationError::InvalidEmail => write!(f, "Invalid email address"),
            VerificationError::NetworkError(e) => write!(f, "Network error: {}", e),
            VerificationError::UnknownError(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl std::error::Error for VerificationError {}

#[async_trait]
pub trait VerificationPort: Send + Sync {
    async fn verify_by_link(
        &self,
        request: VerifyByLinkRequest,
        cookie: Option<&str>,
    ) -> Result<(), VerificationError>;

    async fn send_verification_code(
        &self,
        request: SendCodeRequest,
        cookie: Option<&str>,
    ) -> Result<(), VerificationError>;

    async fn submit_verification_code(
        &self,
        request: SubmitCodeRequest,
        cookie: &str,
    ) -> Result<(), VerificationError>;
}

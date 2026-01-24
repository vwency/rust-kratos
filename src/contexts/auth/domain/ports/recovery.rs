use crate::contexts::auth::domain::graphql::inputs::RecoveryInput;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct RecoveryRequest {
    pub email: String,
}

impl From<RecoveryInput> for RecoveryRequest {
    fn from(input: RecoveryInput) -> Self {
        Self { email: input.email }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub enum RecoveryError {
    FlowNotFound,
    InvalidEmail,
    NetworkError(String),
    UnknownError(String),
}

impl std::fmt::Display for RecoveryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecoveryError::FlowNotFound => write!(f, "Recovery flow not found"),
            RecoveryError::InvalidEmail => write!(f, "Invalid email address"),
            RecoveryError::NetworkError(e) => write!(f, "Network error: {}", e),
            RecoveryError::UnknownError(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl std::error::Error for RecoveryError {}

#[async_trait]
pub trait RecoveryPort: Send + Sync {
    async fn initiate_recovery(
        &self,
        request: RecoveryRequest,
        cookie: Option<&str>,
    ) -> Result<(), RecoveryError>;
}

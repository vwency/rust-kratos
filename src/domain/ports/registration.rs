use crate::domain::graphql::inputs::RegisterInput;
use async_trait::async_trait;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct RegistrationData {
    pub email: String,
    pub username: String,
    pub password: String,
    pub geo_location: Option<String>,
}

impl From<RegisterInput> for RegistrationData {
    fn from(input: RegisterInput) -> Self {
        Self {
            email: input.email,
            username: input.username,
            password: input.password,
            geo_location: input.geo_location,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub enum RegistrationError {
    FlowNotFound,
    InvalidData,
    EmailAlreadyExists,
    NetworkError(String),
    UnknownError(String),
}

impl std::fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistrationError::FlowNotFound => write!(f, "Registration flow not found"),
            RegistrationError::InvalidData => write!(f, "Invalid registration data"),
            RegistrationError::EmailAlreadyExists => write!(f, "Email already exists"),
            RegistrationError::NetworkError(e) => write!(f, "Network error: {}", e),
            RegistrationError::UnknownError(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl std::error::Error for RegistrationError {}

#[async_trait]
pub trait RegistrationPort: Send + Sync {
    async fn initiate_registration(
        &self,
        cookie: Option<&str>,
    ) -> Result<String, RegistrationError>;
    async fn complete_registration(
        &self,
        flow_id: &str,
        data: RegistrationData,
    ) -> Result<String, RegistrationError>;
}

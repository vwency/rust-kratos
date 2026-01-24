use crate::contexts::auth::domain::graphql::inputs::LoginInput;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct LoginCredentials {
    pub identifier: String,
    pub password: String,
    pub address: Option<String>,
    pub code: Option<String>,
    pub resend: Option<String>,
}

impl From<LoginInput> for LoginCredentials {
    fn from(input: LoginInput) -> Self {
        Self {
            identifier: input.identifier,
            password: input.password,
            address: input.address,
            code: input.code,
            resend: input.resend,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub enum AuthError {
    AlreadyLoggedIn,
    InvalidCredentials,
    FlowNotFound,
    NetworkError(String),
    UnknownError(String),
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::AlreadyLoggedIn => write!(f, "Already logged in"),
            AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
            AuthError::FlowNotFound => write!(f, "Flow not found"),
            AuthError::NetworkError(e) => write!(f, "Network error: {}", e),
            AuthError::UnknownError(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl std::error::Error for AuthError {}

#[async_trait]
pub trait AuthenticationPort: Send + Sync {
    async fn initiate_login(&self, cookie: Option<&str>) -> Result<String, AuthError>;
    async fn complete_login(
        &self,
        flow_id: &str,
        credentials: LoginCredentials,
    ) -> Result<String, AuthError>;
}

use async_trait::async_trait;

#[allow(unused)]
#[derive(Debug)]
pub enum SessionError {
    NotAuthenticated,
    InvalidCookie,
    NetworkError(String),
    UnknownError(String),
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::NotAuthenticated => write!(f, "Not authenticated"),
            SessionError::InvalidCookie => write!(f, "Invalid session cookie"),
            SessionError::NetworkError(e) => write!(f, "Network error: {}", e),
            SessionError::UnknownError(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl std::error::Error for SessionError {}

#[async_trait]
pub trait SessionPort: Send + Sync {
    async fn logout(&self, cookie: &str) -> Result<(), SessionError>;
    async fn check_active_session(&self, cookie: Option<&str>) -> bool;
}

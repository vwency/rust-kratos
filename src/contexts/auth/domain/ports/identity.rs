use crate::contexts::auth::domain::entities::user::UserProfile;
use async_trait::async_trait;

#[allow(unused)]
#[derive(Debug)]
pub enum IdentityError {
    NotAuthenticated,
    SessionExpired,
    NetworkError(String),
    UnknownError(String),
}

impl std::fmt::Display for IdentityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentityError::NotAuthenticated => write!(f, "Not authenticated"),
            IdentityError::SessionExpired => write!(f, "Session expired"),
            IdentityError::NetworkError(e) => write!(f, "Network error: {}", e),
            IdentityError::UnknownError(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl std::error::Error for IdentityError {}

#[async_trait]
pub trait IdentityPort: Send + Sync {
    async fn get_current_user(&self, cookie: &str) -> Result<UserProfile, IdentityError>;
}

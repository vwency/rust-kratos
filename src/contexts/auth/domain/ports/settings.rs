use crate::contexts::auth::domain::graphql::inputs::UpdateSettingsInput;
use async_trait::async_trait;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct SettingsData {
    pub method: String,
    pub password: Option<String>,
    pub traits: Option<serde_json::Value>,
    pub lookup_secret_confirm: Option<bool>,
    pub lookup_secret_disable: Option<bool>,
    pub lookup_secret_regenerate: Option<bool>,
    pub lookup_secret_reveal: Option<bool>,
    pub transient_payload: Option<serde_json::Value>,
}

impl From<UpdateSettingsInput> for SettingsData {
    fn from(input: UpdateSettingsInput) -> Self {
        Self {
            method: input.method,
            password: input.password,
            traits: input.traits,
            lookup_secret_confirm: input.lookup_secret_confirm,
            lookup_secret_disable: input.lookup_secret_disable,
            lookup_secret_regenerate: input.lookup_secret_regenerate,
            lookup_secret_reveal: input.lookup_secret_reveal,
            transient_payload: input.transient_payload,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub enum SettingsError {
    FlowNotFound,
    InvalidData,
    Unauthorized,
    PrivilegedSessionRequired,
    NetworkError(String),
    UnknownError(String),
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsError::FlowNotFound => write!(f, "Settings flow not found"),
            SettingsError::InvalidData => write!(f, "Invalid settings data"),
            SettingsError::Unauthorized => write!(f, "Unauthorized access"),
            SettingsError::PrivilegedSessionRequired => write!(f, "Privileged session required"),
            SettingsError::NetworkError(e) => write!(f, "Network error: {}", e),
            SettingsError::UnknownError(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl std::error::Error for SettingsError {}

#[async_trait]
pub trait SettingsPort: Send + Sync {
    async fn initiate_settings(&self, cookie: &str) -> Result<String, SettingsError>;
    async fn update_settings(
        &self,
        flow_id: &str,
        data: SettingsData,
        cookie: &str,
    ) -> Result<String, SettingsError>;
}

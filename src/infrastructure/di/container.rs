use crate::application::config::Config;
use crate::application::usecases::auth::get_current_user::GetCurrentUserUseCase;
use crate::application::usecases::auth::login::LoginUseCase;
use crate::application::usecases::auth::logout::LogoutUseCase;
use crate::application::usecases::auth::recovery::RecoveryUseCase;
use crate::application::usecases::auth::register::RegisterUseCase;
use crate::application::usecases::auth::verification::VerificationUseCase;
use crate::infrastructure::adapters::kratos::KratosClient;
use crate::infrastructure::di::factory::KratosAdapterFactory;
use std::sync::Arc;

pub struct UseCases {
    pub register: Arc<RegisterUseCase>,
    pub login: Arc<LoginUseCase>,
    pub logout: Arc<LogoutUseCase>,
    pub recovery: Arc<RecoveryUseCase>,
    pub verification: Arc<VerificationUseCase>,
    pub get_current_user: Arc<GetCurrentUserUseCase>,
}

impl UseCases {
    fn new(factory: Arc<KratosAdapterFactory>) -> Self {
        Self {
            register: Arc::new(RegisterUseCase::new(factory.create_registration_adapter())),
            login: Arc::new(LoginUseCase::new(factory.create_authentication_adapter())),
            logout: Arc::new(LogoutUseCase::new(factory.create_session_adapter())),
            recovery: Arc::new(RecoveryUseCase::new(factory.create_recovery_adapter())),
            verification: Arc::new(VerificationUseCase::new(
                factory.create_verification_adapter(),
            )),
            get_current_user: Arc::new(GetCurrentUserUseCase::new(
                factory.create_identity_adapter(),
            )),
        }
    }
}

#[derive(Clone)]
pub struct AppContainer {
    pub use_cases: Arc<UseCases>,
    _factory: Arc<KratosAdapterFactory>,
}

impl AppContainer {
    pub fn new(config: &Config) -> Result<Self, ContainerError> {
        Self::validate_config(config)?;

        let kratos_client = Arc::new(KratosClient::new(&config.kratos));
        let factory = Arc::new(KratosAdapterFactory::new(kratos_client));
        let use_cases = Arc::new(UseCases::new(factory.clone()));

        Ok(Self {
            use_cases,
            _factory: factory,
        })
    }

    fn validate_config(config: &Config) -> Result<(), ContainerError> {
        if config.kratos.public_url.is_empty() {
            return Err(ContainerError::InvalidConfiguration(
                "Kratos public URL cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ContainerError {
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
    #[error("Initialization failed: {0}")]
    InitializationFailed(String),
}

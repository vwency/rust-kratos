use crate::bootstrap::config::Config;
use crate::contexts::auth::application::usecases::auth::get_current_user::GetCurrentUserUseCase;
use crate::contexts::auth::application::usecases::auth::login::LoginUseCase;
use crate::contexts::auth::application::usecases::auth::logout::LogoutUseCase;
use crate::contexts::auth::application::usecases::auth::recovery::RecoveryUseCase;
use crate::contexts::auth::application::usecases::auth::register::RegisterUseCase;
use crate::contexts::auth::application::usecases::auth::verification::VerificationUseCase;
use crate::infrastructure::di::adapter_factory::AdapterFactory;
use std::sync::Arc;

pub struct UseCases {
    pub register: RegisterUseCase,
    pub login: LoginUseCase,
    pub logout: LogoutUseCase,
    pub recovery: RecoveryUseCase,
    pub verification: VerificationUseCase,
    pub get_current_user: GetCurrentUserUseCase,
}

impl UseCases {
    pub fn new(factory: &dyn AdapterFactory) -> Self {
        Self {
            register: RegisterUseCase::new(factory.create_registration_adapter()),
            login: LoginUseCase::new(factory.create_authentication_adapter()),
            logout: LogoutUseCase::new(factory.create_session_adapter()),
            recovery: RecoveryUseCase::new(factory.create_recovery_adapter()),
            verification: VerificationUseCase::new(factory.create_verification_adapter()),
            get_current_user: GetCurrentUserUseCase::new(factory.create_identity_adapter()),
        }
    }
}

#[derive(Clone)]
pub struct AppContainer {
    pub use_cases: Arc<UseCases>,
}

impl AppContainer {
    pub fn new(config: &Config) -> Result<Self, ContainerError> {
        Self::validate_config(config)?;
        let factory = Self::create_factory(config)?;
        let use_cases = Arc::new(UseCases::new(factory.as_ref()));
        Ok(Self { use_cases })
    }

    fn create_factory(config: &Config) -> Result<Box<dyn AdapterFactory>, ContainerError> {
        crate::infrastructure::di::factory::KratosAdapterFactory::new(&config.kratos)
            .map(|f| Box::new(f) as Box<dyn AdapterFactory>)
            .map_err(|e| ContainerError::FactoryCreationFailed(e.to_string()))
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
    #[error("Factory creation failed: {0}")]
    FactoryCreationFailed(String),
}

use crate::application::config::Config;
use crate::application::usecases::auth::get_current_user::GetCurrentUserUseCase;
use crate::application::usecases::auth::login::LoginUseCase;
use crate::application::usecases::auth::logout::LogoutUseCase;
use crate::application::usecases::auth::recovery::RecoveryUseCase;
use crate::application::usecases::auth::register::RegisterUseCase;
use crate::application::usecases::auth::verification::VerificationUseCase;
use crate::infrastructure::adapters::kratos::KratosClient;
use crate::infrastructure::adapters::kratos::http::identity::KratosIdentityAdapter;
use crate::infrastructure::adapters::kratos::http::login::KratosAuthenticationAdapter;
use crate::infrastructure::adapters::kratos::http::logout::KratosSessionAdapter;
use crate::infrastructure::adapters::kratos::http::recovery::KratosRecoveryAdapter;
use crate::infrastructure::adapters::kratos::http::register::KratosRegistrationAdapter;
use crate::infrastructure::adapters::kratos::http::verification::KratosVerificationAdapter;
use std::sync::Arc;

struct Adapters {
    registration: KratosRegistrationAdapter,
    authentication: KratosAuthenticationAdapter,
    session: KratosSessionAdapter,
    recovery: KratosRecoveryAdapter,
    verification: KratosVerificationAdapter,
    identity: KratosIdentityAdapter,
}

impl Adapters {
    fn new(kratos_client: Arc<KratosClient>) -> Self {
        Self {
            registration: KratosRegistrationAdapter::new(kratos_client.clone()),
            authentication: KratosAuthenticationAdapter::new(kratos_client.clone()),
            session: KratosSessionAdapter::new(kratos_client.clone()),
            recovery: KratosRecoveryAdapter::new(kratos_client.clone()),
            verification: KratosVerificationAdapter::new(kratos_client.clone()),
            identity: KratosIdentityAdapter::new(kratos_client),
        }
    }
}

pub struct UseCases {
    pub register: Arc<RegisterUseCase>,
    pub login: Arc<LoginUseCase>,
    pub logout: Arc<LogoutUseCase>,
    pub recovery: Arc<RecoveryUseCase>,
    pub verification: Arc<VerificationUseCase>,
    pub get_current_user: Arc<GetCurrentUserUseCase>,
}

impl UseCases {
    fn new(adapters: Adapters) -> Self {
        Self {
            register: Arc::new(RegisterUseCase::new(Box::new(adapters.registration))),
            login: Arc::new(LoginUseCase::new(Box::new(adapters.authentication))),
            logout: Arc::new(LogoutUseCase::new(Box::new(adapters.session))),
            recovery: Arc::new(RecoveryUseCase::new(Box::new(adapters.recovery))),
            verification: Arc::new(VerificationUseCase::new(Box::new(adapters.verification))),
            get_current_user: Arc::new(GetCurrentUserUseCase::new(Box::new(adapters.identity))),
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

        let kratos_client = Arc::new(KratosClient::new(&config.kratos));
        let adapters = Adapters::new(kratos_client);
        let use_cases = Arc::new(UseCases::new(adapters));

        Ok(Self { use_cases })
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

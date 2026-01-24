use crate::domain::ports::auth::AuthenticationPort;
use crate::domain::ports::identity::IdentityPort;
use crate::domain::ports::recovery::RecoveryPort;
use crate::domain::ports::registration::RegistrationPort;
use crate::domain::ports::session::SessionPort;
use crate::domain::ports::verification::VerificationPort;
use crate::infrastructure::adapters::kratos::KratosClient;
use crate::infrastructure::adapters::kratos::http::identity::KratosIdentityAdapter;
use crate::infrastructure::adapters::kratos::http::login::KratosAuthenticationAdapter;
use crate::infrastructure::adapters::kratos::http::logout::KratosSessionAdapter;
use crate::infrastructure::adapters::kratos::http::recovery::KratosRecoveryAdapter;
use crate::infrastructure::adapters::kratos::http::register::KratosRegistrationAdapter;
use crate::infrastructure::adapters::kratos::http::verification::KratosVerificationAdapter;
use std::sync::Arc;

pub struct KratosAdapterFactory {
    client: Arc<KratosClient>,
}

impl KratosAdapterFactory {
    pub fn new(client: Arc<KratosClient>) -> Self {
        Self { client }
    }

    pub fn create_registration_adapter(&self) -> Box<dyn RegistrationPort> {
        Box::new(KratosRegistrationAdapter::new(self.client.clone()))
    }

    pub fn create_authentication_adapter(&self) -> Box<dyn AuthenticationPort> {
        Box::new(KratosAuthenticationAdapter::new(self.client.clone()))
    }

    pub fn create_session_adapter(&self) -> Box<dyn SessionPort> {
        Box::new(KratosSessionAdapter::new(self.client.clone()))
    }

    pub fn create_recovery_adapter(&self) -> Box<dyn RecoveryPort> {
        Box::new(KratosRecoveryAdapter::new(self.client.clone()))
    }

    pub fn create_verification_adapter(&self) -> Box<dyn VerificationPort> {
        Box::new(KratosVerificationAdapter::new(self.client.clone()))
    }

    pub fn create_identity_adapter(&self) -> Box<dyn IdentityPort> {
        Box::new(KratosIdentityAdapter::new(self.client.clone()))
    }
}

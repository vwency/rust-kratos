use crate::domain::ports::{
    auth::AuthenticationPort, identity::IdentityPort, recovery::RecoveryPort,
    registration::RegistrationPort, session::SessionPort, verification::VerificationPort,
};
use std::sync::Arc;

pub trait AdapterFactory: Send + Sync {
    fn create_registration_adapter(&self) -> Arc<dyn RegistrationPort>;
    fn create_authentication_adapter(&self) -> Arc<dyn AuthenticationPort>;
    fn create_session_adapter(&self) -> Arc<dyn SessionPort>;
    fn create_recovery_adapter(&self) -> Arc<dyn RecoveryPort>;
    fn create_verification_adapter(&self) -> Arc<dyn VerificationPort>;
    fn create_identity_adapter(&self) -> Arc<dyn IdentityPort>;
}

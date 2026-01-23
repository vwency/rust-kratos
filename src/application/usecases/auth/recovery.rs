use crate::domain::graphql::inputs::RecoveryInput;
use crate::domain::ports::{RecoveryError, RecoveryPort};
use tracing::{error, info};

pub struct RecoveryUseCase {
    recovery_port: Box<dyn RecoveryPort>,
}

impl RecoveryUseCase {
    pub fn new(recovery_port: Box<dyn RecoveryPort>) -> Self {
        Self { recovery_port }
    }

    pub async fn execute(
        &self,
        input: RecoveryInput,
        cookie: Option<&str>,
    ) -> Result<(), RecoveryError> {
        info!(
            email = &input.email,
            cookie_present = cookie.is_some(),
            "Starting recovery process"
        );

        self.recovery_port
            .initiate_recovery(input.into(), cookie)
            .await
            .map_err(|e| {
                error!(error = %e, "Recovery failed");
                e
            })?;

        info!("Recovery email sent successfully");
        Ok(())
    }
}

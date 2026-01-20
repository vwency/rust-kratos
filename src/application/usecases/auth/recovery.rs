use crate::domain::auth::inputs::RecoveryInput;
use crate::infrastructure::adapters::kratos::KratosClient;
use tracing::{debug, error, info};

pub struct RecoveryUseCase;

impl RecoveryUseCase {
    pub async fn execute(
        input: RecoveryInput,
        kratos_client: &KratosClient,
        cookie: Option<&str>,
    ) -> Result<Vec<String>, String> {
        info!(
            email = &input.email,
            cookie_present = cookie.is_some(),
            "Starting recovery process"
        );

        let cookies = match kratos_client.recovery(&input.email, cookie).await {
            Ok(result) => result,
            Err(e) => {
                let error_msg = e.to_string();
                error!(error = %error_msg, "Recovery failed");
                return Err(format!("Recovery failed: {}", error_msg));
            }
        };

        if cookies.is_empty() {
            debug!("No cookies returned from Kratos");
        } else {
            debug!(
                cookies_count = cookies.len(),
                cookies = ?cookies,
                "Cookies returned from Kratos"
            );
        }

        info!(
            "Recovery email sent successfully for email={}",
            &input.email
        );
        Ok(cookies)
    }
}

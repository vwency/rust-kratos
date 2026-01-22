use crate::domain::auth::inputs::LoginInput;
use crate::infrastructure::adapters::kratos::KratosClient;
use tracing::{debug, error, info};

pub struct LoginUseCase;

impl LoginUseCase {
    pub async fn execute(
        input: LoginInput,
        kratos_client: &KratosClient,
        cookie: Option<&str>,
    ) -> Result<Vec<String>, String> {
        let identifier = input
            .email
            .as_ref()
            .or(input.username.as_ref())
            .ok_or("Email or username required")?;

        info!(
            identifier = identifier,
            cookie_present = cookie.is_some(),
            "Starting login process"
        );

        if let Some(cookie) = cookie {
            if let Ok(Some(_session)) = kratos_client.get_session(cookie).await {
                error!("Login attempt with active session for {}", identifier);
                return Err(
                    "Already logged in. Please logout first before logging in again.".to_string(),
                );
            }
        }

        let cookies = match kratos_client
            .login(
                identifier,
                &input.password,
                input.address.as_deref(),
                input.code.as_deref(),
                input.identifier.as_deref(),
                input.resend.as_deref(),
                cookie,
            )
            .await
        {
            Ok(result) => result,
            Err(e) => {
                let error_msg = e.to_string();
                error!(error = %error_msg, "Login failed");
                return Err(format!("Login failed: {}", error_msg));
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

        info!("Login successful for identifier={}", identifier);
        Ok(cookies)
    }
}

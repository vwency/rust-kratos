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
        let identifier = &input.identifier;
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

        let flow = match kratos_client.get_login_flow(cookie).await {
            Ok(flow) => flow,
            Err(e) => {
                let error_msg = e.to_string();
                error!(error = %error_msg, "Failed to get login flow");
                return Err(format!("Failed to get login flow: {}", error_msg));
            }
        };

        let flow_id = flow.flow["id"]
            .as_str()
            .ok_or("Flow ID not found")?
            .to_string();

        let cookies = match kratos_client
            .submit_login_flow(
                &flow_id,
                &flow.csrf_token,
                &input.identifier,
                &input.password,
                input.address.as_deref(),
                input.code.as_deref(),
                input.resend.as_deref(),
                &flow.cookies,
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

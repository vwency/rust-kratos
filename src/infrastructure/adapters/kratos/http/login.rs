use crate::domain::ports::auth::{AuthError, AuthenticationPort, LoginCredentials};
use crate::domain::ports::session::SessionPort;
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::http::flows::{fetch_flow, post_flow};
use crate::infrastructure::adapters::kratos::http::logout::KratosSessionAdapter;
use async_trait::async_trait;
use tracing::{debug, error};

#[allow(unused)]
pub struct KratosAuthenticationAdapter {
    client: KratosClient,
    session_adapter: KratosSessionAdapter,
}

#[allow(unused)]
impl KratosAuthenticationAdapter {
    pub fn new(client: KratosClient) -> Self {
        let session_adapter = KratosSessionAdapter::new(client.clone());
        Self {
            client,
            session_adapter,
        }
    }
}

#[async_trait]
impl AuthenticationPort for KratosAuthenticationAdapter {
    async fn initiate_login(&self, cookie: Option<&str>) -> Result<String, AuthError> {
        if self.session_adapter.check_active_session(cookie).await {
            error!("Login attempt with an already active session");
            return Err(AuthError::AlreadyLoggedIn);
        }

        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "login",
            cookie,
        )
        .await
        .map_err(|e| AuthError::NetworkError(e.to_string()))?;

        flow.flow["id"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| AuthError::FlowNotFound)
    }

    async fn complete_login(
        &self,
        flow_id: &str,
        credentials: LoginCredentials,
    ) -> Result<String, AuthError> {
        let flow = fetch_flow(&self.client.client, &self.client.public_url, "login", None)
            .await
            .map_err(|e| AuthError::NetworkError(e.to_string()))?;

        let csrf_token = flow.csrf_token.clone();

        debug!("Using flow_id: {}, csrf_token: {}", flow_id, csrf_token);

        let mut payload = serde_json::json!({
            "method": "password",
            "identifier": credentials.identifier,
            "password": credentials.password,
            "csrf_token": csrf_token,
        });

        if let Some(addr) = credentials.address {
            payload["address"] = serde_json::json!(addr);
        }

        if let Some(code) = credentials.code {
            payload["code"] = serde_json::json!(code);
            payload["method"] = serde_json::json!("code");
        }

        if let Some(resend) = credentials.resend {
            payload["resend"] = serde_json::json!(resend);
        }

        debug!(
            "Login payload: {}",
            serde_json::to_string_pretty(&payload).unwrap()
        );

        let result = post_flow(
            &self.client.client,
            &self.client.public_url,
            "login",
            flow_id,
            payload,
            &flow.cookies,
        )
        .await
        .map_err(|e| {
            error!("Failed to post login flow: {}", e);
            AuthError::NetworkError(e.to_string())
        })?;

        debug!("Received cookies: {:?}", result.cookies);
        debug!("Response data: {:?}", result.data);

        if let Some(session) = result.data.get("session") {
            debug!("Session data present: {:?}", session);
        }

        if result.cookies.is_empty() {
            error!("No cookies in response");
            return Err(AuthError::UnknownError(
                "No cookies received from server".to_string(),
            ));
        }

        result
            .cookies
            .into_iter()
            .find(|c| c.contains("session") || c.starts_with("ory_"))
            .ok_or_else(|| {
                error!("Session cookie not found in response cookies");
                AuthError::UnknownError("Session token not found".to_string())
            })
    }
}

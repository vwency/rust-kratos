use crate::domain::ports::auth::{AuthError, AuthenticationPort, LoginCredentials};
use crate::domain::ports::session::SessionPort;
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::flows::{fetch_flow, post_flow};
use crate::infrastructure::adapters::kratos::handlers::logout::KratosSessionAdapter;
use async_trait::async_trait;
use tracing::error;

pub struct KratosAuthenticationAdapter {
    client: KratosClient,
    session_adapter: KratosSessionAdapter,
}

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

        let mut payload = serde_json::json!({
            "method": "password",
            "identifier": credentials.identifier,
            "password": credentials.password,
            "csrf_token": flow.csrf_token,
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

        let result = post_flow(
            &self.client.client,
            &self.client.public_url,
            "login",
            flow_id,
            payload,
            &flow.cookies,
        )
        .await
        .map_err(|e| AuthError::NetworkError(e.to_string()))?;

        result
            .cookies
            .into_iter()
            .find(|c| c.starts_with("ory_session_"))
            .ok_or_else(|| AuthError::UnknownError("Session token not found".to_string()))
    }
}

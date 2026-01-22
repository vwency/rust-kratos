use crate::application::usecases::auth::verification::VerificationUseCase;
use crate::infrastructure::adapters::graphql::cookies::ResponseCookies;
use crate::infrastructure::adapters::kratos::KratosClient;
use async_graphql::{Context, InputObject, Object, Result};
use serde_json::Value;

#[derive(InputObject)]
pub struct VerificationInput {
    pub email: Option<String>,
    pub code: Option<String>,
    pub transient_payload: Option<Value>,
}

#[derive(Default)]
pub struct VerificationMutation;

#[Object]
impl VerificationMutation {
    async fn verify(&self, ctx: &Context<'_>, input: VerificationInput) -> Result<bool> {
        let kratos_client = ctx.data_unchecked::<KratosClient>();
        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        let cookies = match (input.email, input.code) {
            (Some(email), None) => {
                VerificationUseCase::execute_with_email(
                    &email,
                    kratos_client,
                    cookie,
                    input.transient_payload,
                )
                .await
            }
            (None, Some(code)) => {
                VerificationUseCase::execute_with_code(
                    &code,
                    kratos_client,
                    cookie,
                    input.transient_payload,
                )
                .await
            }
            _ => {
                return Err(async_graphql::Error::new(
                    "Either email or code must be provided, but not both",
                ));
            }
        }
        .map_err(async_graphql::Error::new)?;

        if let Some(response_cookies) = ctx.data_opt::<ResponseCookies>() {
            for cookie_str in cookies {
                response_cookies.add_cookie(cookie_str).await;
            }
        }

        Ok(true)
    }
}

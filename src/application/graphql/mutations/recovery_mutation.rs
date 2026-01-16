use crate::application::usecases::auth::recovery::RecoveryUseCase;
use crate::domain::auth::inputs::RecoveryInput;
use crate::infrastructure::adapters::graphql::cookies::ResponseCookies;
use crate::infrastructure::adapters::kratos::KratosClient;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct RecoveryMutation;

#[Object]
impl RecoveryMutation {
    async fn recovery(&self, ctx: &Context<'_>, input: RecoveryInput) -> Result<bool> {
        let kratos_client = ctx.data_unchecked::<KratosClient>();
        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        let cookies = RecoveryUseCase::execute(input, kratos_client, cookie)
            .await
            .map_err(async_graphql::Error::new)?;

        if let Some(response_cookies) = ctx.data_opt::<ResponseCookies>() {
            for cookie_str in cookies {
                response_cookies.add_cookie(cookie_str).await;
            }
        }

        Ok(true)
    }
}

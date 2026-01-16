use crate::application::usecases::auth::login::LoginUseCase;
use crate::domain::auth::inputs::LoginInput;
use crate::infrastructure::adapters::graphql::cookies::ResponseCookies;
use crate::infrastructure::adapters::kratos::KratosClient;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct LoginMutation;

#[Object]
impl LoginMutation {
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<bool> {
        let kratos_client = ctx.data_unchecked::<KratosClient>();

        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        let cookies = LoginUseCase::execute(input, kratos_client, cookie)
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

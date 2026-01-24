use crate::contexts::auth::application::usecases::auth::login::LoginUseCase;
use crate::contexts::auth::domain::graphql::inputs::LoginInput;
use crate::infrastructure::adapters::graphql::cookies::ResponseCookies;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct LoginMutation;

#[Object]
impl LoginMutation {
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<bool> {
        let login_use_case = ctx.data_unchecked::<LoginUseCase>();

        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        let session_token = login_use_case
            .execute(input, cookie)
            .await
            .map_err(async_graphql::Error::new)?;

        if let Some(response_cookies) = ctx.data_opt::<ResponseCookies>() {
            response_cookies.add_cookie(session_token).await;
        }

        Ok(true)
    }
}

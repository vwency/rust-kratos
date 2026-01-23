use crate::application::usecases::auth::register::RegisterUseCase;
use crate::domain::graphql::inputs::RegisterInput;
use crate::infrastructure::adapters::graphql::cookies::ResponseCookies;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct RegisterMutation;

#[Object]
impl RegisterMutation {
    async fn register(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<bool> {
        let register_use_case = ctx.data_unchecked::<RegisterUseCase>();

        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        let session_token = register_use_case
            .execute(input, cookie)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        if let Some(response_cookies) = ctx.data_opt::<ResponseCookies>() {
            response_cookies.add_cookie(session_token).await;
        }

        Ok(true)
    }
}

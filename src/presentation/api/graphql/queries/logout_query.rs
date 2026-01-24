use crate::contexts::auth::application::usecases::auth::logout::LogoutUseCase;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct LogoutQuery;

#[Object]
impl LogoutQuery {
    async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
        let logout_use_case = ctx.data_unchecked::<LogoutUseCase>();

        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        logout_use_case.execute(cookie).await?;

        Ok(true)
    }
}

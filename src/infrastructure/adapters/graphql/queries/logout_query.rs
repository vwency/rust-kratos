use crate::application::usecases::auth::logout::LogoutUseCase;
use crate::infrastructure::adapters::kratos::KratosClient;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct LogoutQuery;

#[Object]
impl LogoutQuery {
    async fn logout(&self, ctx: &Context<'_>) -> Result<Vec<String>> {
        let kratos_client = ctx.data_unchecked::<KratosClient>();

        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        let cleared_cookies = LogoutUseCase::execute(kratos_client, cookie)
            .await
            .map_err(async_graphql::Error::new)?;

        Ok(cleared_cookies)
    }
}

use crate::application::usecases::auth::get_current_user::GetCurrentUserUseCase;
use crate::domain::kratos::models::IdentityTraits;
use crate::infrastructure::adapters::kratos::KratosClient;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct CurrentUserQuery;

#[Object]
impl CurrentUserQuery {
    async fn current_user(&self, ctx: &Context<'_>) -> Result<IdentityTraits> {
        let kratos_client = ctx.data_unchecked::<KratosClient>();

        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        let traits = GetCurrentUserUseCase::execute(kratos_client, cookie)
            .await
            .map_err(async_graphql::Error::new)?;

        Ok(traits)
    }
}

use crate::contexts::auth::application::usecases::auth::recovery::RecoveryUseCase;
use crate::contexts::auth::domain::graphql::inputs::RecoveryInput;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct RecoveryMutation;

#[Object]
impl RecoveryMutation {
    async fn recovery(&self, ctx: &Context<'_>, input: RecoveryInput) -> Result<bool> {
        let recovery_use_case = ctx.data_unchecked::<RecoveryUseCase>();

        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        recovery_use_case.execute(input, cookie).await?;

        Ok(true)
    }
}

use crate::contexts::auth::application::usecases::auth::get_current_user::GetCurrentUserUseCase;
use crate::contexts::auth::domain::entities::user::UserProfile;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct CurrentUserQuery;

#[Object]
impl CurrentUserQuery {
    async fn current_user(&self, ctx: &Context<'_>) -> Result<UserProfile> {
        let get_current_user_use_case = ctx.data_unchecked::<GetCurrentUserUseCase>();

        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        let user_profile = get_current_user_use_case.execute(cookie).await?;

        Ok(user_profile)
    }
}

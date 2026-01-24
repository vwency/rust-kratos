use crate::domain::graphql::inputs::RegisterInput;
use crate::infrastructure::adapters::graphql::cookies::ResponseCookies;
use crate::infrastructure::di::container::UseCases;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct RegisterMutation;

#[Object]
impl RegisterMutation {
    async fn register(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<bool> {
        let use_cases = ctx.data_unchecked::<Arc<UseCases>>();
        let result = use_cases
            .register
            .execute(input.into())
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        if let Some(response_cookies) = ctx.data_opt::<ResponseCookies>() {
            response_cookies.add_cookie(result.session_cookie).await;
        }
        Ok(true)
    }
}

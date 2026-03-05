use crate::domain::graphql::inputs::UpdateSettingsInput;
use crate::infrastructure::di::container::UseCases;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct SettingsMutation;

#[Object]
impl SettingsMutation {
    async fn update_settings(
        &self,
        ctx: &Context<'_>,
        input: UpdateSettingsInput,
    ) -> Result<String> {
        let use_cases = ctx.data_unchecked::<Arc<UseCases>>();

        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str())
            .unwrap_or_default();

        use_cases
            .update_settings
            .execute(input, cookie)
            .await
            .map_err(async_graphql::Error::new)
    }
}

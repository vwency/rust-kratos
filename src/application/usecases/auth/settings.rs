use crate::domain::graphql::inputs::UpdateSettingsInput;
use crate::domain::ports::settings::{SettingsData, SettingsPort};
use std::sync::Arc;

pub struct UpdateSettingsUseCase {
    settings_port: Arc<dyn SettingsPort>,
}

impl UpdateSettingsUseCase {
    pub fn new(settings_port: Arc<dyn SettingsPort>) -> Self {
        Self { settings_port }
    }

    pub async fn execute(
        &self,
        input: UpdateSettingsInput,
        cookie: &str,
    ) -> Result<(String, Vec<String>), String> {
        let flow_id = self
            .settings_port
            .initiate_settings(cookie)
            .await
            .map_err(|e| e.to_string())?;

        let data = SettingsData::from(input);

        self.settings_port
            .update_settings(&flow_id, data, cookie)
            .await
            .map_err(|e| e.to_string())
    }
}

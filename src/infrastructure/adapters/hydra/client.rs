use crate::application::bootstrap::config::HydraConfig;
use reqwest::Client;
use std::time::Duration;

#[derive(Clone)]
pub struct HydraClient {
    pub(crate) client: Client,
    pub(crate) admin_url: String,
}

impl HydraClient {
    pub fn new(config: &HydraConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .connect_timeout(Duration::from_secs(config.connect_timeout_secs))
            .danger_accept_invalid_certs(config.accept_invalid_certs)
            .build()
            .expect("Failed to build Hydra HTTP client");
        Self {
            client,
            admin_url: config.admin_url.clone(),
        }
    }
}

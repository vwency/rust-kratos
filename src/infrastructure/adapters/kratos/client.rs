use crate::application::bootstrap::config::KratosConfig;
use reqwest::Client;
use std::time::Duration;

#[derive(Clone)]
pub struct KratosClient {
    #[allow(unused)]
    pub(crate) client: Client,
    #[allow(unused)]
    pub(crate) admin_url: String,
    #[allow(unused)]
    pub(crate) public_url: String,
    #[allow(unused)]
    pub(crate) max_retries: u32,
    #[allow(unused)]
    pub(crate) retry_delay: Duration,
}

impl KratosClient {
    pub fn new(config: &KratosConfig) -> Self {
        let client = Client::builder()
            .cookie_store(false)
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(config.timeout_secs))
            .connect_timeout(Duration::from_secs(config.connect_timeout_secs))
            .pool_idle_timeout(Duration::from_secs(config.pool_idle_timeout_secs))
            .pool_max_idle_per_host(config.pool_max_idle_per_host)
            .danger_accept_invalid_certs(config.accept_invalid_certs)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            admin_url: config.admin_url.clone(),
            public_url: config.public_url.clone(),
            max_retries: config.max_retries,
            retry_delay: Duration::from_millis(config.retry_delay_ms),
        }
    }

    #[allow(unused)]
    pub async fn execute_with_retry<F, Fut, T, E>(&self, mut operation: F) -> Result<T, E>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Display,
    {
        let mut attempts = 0;
        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempts += 1;
                    if attempts >= self.max_retries {
                        return Err(e);
                    }
                    tracing::warn!(
                        "Request failed (attempt {}/{}): {}. Retrying in {:?}...",
                        attempts,
                        self.max_retries,
                        e,
                        self.retry_delay
                    );
                    tokio::time::sleep(self.retry_delay).await;
                }
            }
        }
    }
}

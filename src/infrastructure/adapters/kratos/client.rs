use reqwest::Client;
use std::time::Duration;

#[derive(Clone)]
pub struct KratosClient {
    pub(crate) client: Client,
    pub(crate) admin_url: String,
    pub(crate) public_url: String,
}

impl KratosClient {
    pub fn new(admin_url: String, public_url: String) -> Self {
        let client = Client::builder()
            .cookie_store(false)
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(120))
            .connect_timeout(Duration::from_secs(30))
            .pool_idle_timeout(Duration::from_secs(120))
            .pool_max_idle_per_host(10)
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            admin_url,
            public_url,
        }
    }
}

use crate::infrastructure::adapters::kratos::client::KratosClient;

impl KratosClient {
    pub(crate) async fn check_active_session(&self, cookie: Option<&str>) -> bool {
        if let Some(cookie_value) = cookie {
            if let Ok(_) = self.handle_get_current_user(cookie_value).await {
                return true;
            }
        }
        false
    }
}

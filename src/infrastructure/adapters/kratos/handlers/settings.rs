use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::flows::post_flow;

impl KratosClient {
    pub async fn submit_settings_flow(
        &self,
        flow_id: &str,
        csrf_token: &str,
        flow_cookies: Vec<String>,
        traits: Option<serde_json::Value>,
        password: Option<&str>,
        address: Option<&str>,
        code: Option<&str>,
        resend: Option<&str>,
        method: Option<&str>,
        totp_code: Option<&str>,
        totp_unlink: Option<bool>,
        webauthn_remove: Option<&str>,
        lookup_remove: Option<&str>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut settings_data = serde_json::json!({
            "csrf_token": csrf_token,
        });

        if let Some(t) = traits {
            settings_data["traits"] = t;
            settings_data["method"] = serde_json::json!("profile");
        }

        if let Some(pw) = password {
            settings_data["password"] = serde_json::json!(pw);
            settings_data["method"] = serde_json::json!("password");
        }

        if let Some(addr) = address {
            settings_data["address"] = serde_json::json!(addr);
            settings_data["method"] = serde_json::json!("profile");
        }

        if let Some(c) = code {
            settings_data["code"] = serde_json::json!(c);
            settings_data["method"] = serde_json::json!("code");
        }

        if let Some(r) = resend {
            settings_data["resend"] = serde_json::json!(r);
        }

        if let Some(m) = method {
            settings_data["method"] = serde_json::json!(m);
        }

        if let Some(tc) = totp_code {
            settings_data["totp_code"] = serde_json::json!(tc);
        }

        if let Some(unlink) = totp_unlink {
            settings_data["totp_unlink"] = serde_json::json!(unlink);
        }

        if let Some(wr) = webauthn_remove {
            settings_data["webauthn_remove"] = serde_json::json!(wr);
        }

        if let Some(lr) = lookup_remove {
            settings_data["lookup_remove"] = serde_json::json!(lr);
        }

        let post_result = post_flow(
            &self.client,
            &self.public_url,
            "settings",
            flow_id,
            settings_data,
            &flow_cookies,
        )
        .await?;

        Ok(post_result.cookies)
    }
}

use crate::contexts::auth::infrastructure::adapters::kratos::models::flows::{
    FlowResult, PostFlowResult,
};
use reqwest::{Client, StatusCode, header};
use serde_json;
use std::error::Error;

pub async fn fetch_flow(
    client: &Client,
    public_url: &str,
    endpoint: &str,
    cookie: Option<&str>,
) -> Result<FlowResult, Box<dyn Error>> {
    let url = format!("{}/self-service/{}/browser", public_url, endpoint);
    let url = url.replace("localhost", "127.0.0.1");

    let mut request = client.get(&url);

    if let Some(cookie_value) = cookie {
        request = request.header(header::COOKIE, cookie_value);
    }

    let response = request.send().await.map_err(|e| {
        format!(
            "Failed to connect to Kratos at {}: {}. Make sure Kratos is running.",
            url, e
        )
    })?;

    let status = response.status();
    let flow_cookies: Vec<String> = response
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .collect();

    if status == StatusCode::SEE_OTHER || status == StatusCode::FOUND {
        let location = response
            .headers()
            .get(header::LOCATION)
            .and_then(|h| h.to_str().ok())
            .ok_or("No redirect location found")?;

        let flow_id = location
            .split("flow=")
            .nth(1)
            .ok_or("Flow ID not found in redirect URL")?;

        let flow_url = format!(
            "{}/self-service/{}/flows?id={}",
            public_url.replace("localhost", "127.0.0.1"),
            endpoint,
            flow_id
        );

        let mut flow_request = client.get(&flow_url);

        if !flow_cookies.is_empty() {
            flow_request = flow_request.header(header::COOKIE, flow_cookies.join("; "));
        } else if let Some(cookie_value) = cookie {
            flow_request = flow_request.header(header::COOKIE, cookie_value);
        }

        let flow_response = flow_request.send().await?;

        if !flow_response.status().is_success() {
            let status = flow_response.status();
            let error_text = flow_response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            return Err(format!(
                "Failed to fetch {} flow (status {}): {}",
                endpoint, status, error_text
            )
            .into());
        }

        let flow: serde_json::Value = flow_response
            .json()
            .await
            .map_err(|e| format!("Failed to parse {} flow response: {}", endpoint, e))?;

        let csrf_token = flow["ui"]["nodes"]
            .as_array()
            .and_then(|nodes| {
                nodes
                    .iter()
                    .find(|node| node["attributes"]["name"].as_str() == Some("csrf_token"))
            })
            .and_then(|node| node["attributes"]["value"].as_str())
            .ok_or("CSRF token not found in flow response")?
            .to_string();

        let mut all_cookies = Vec::new();
        if let Some(existing_cookie) = cookie {
            all_cookies.push(existing_cookie.to_string());
        }
        all_cookies.extend(flow_cookies);

        return Ok(FlowResult {
            flow,
            csrf_token,
            cookies: all_cookies,
        });
    }

    if !status.is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!(
            "Failed to fetch {} flow (status {}): {}",
            endpoint, status, error_text
        )
        .into());
    }

    let flow: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse {} flow response: {}", endpoint, e))?;

    let csrf_token = flow["ui"]["nodes"]
        .as_array()
        .and_then(|nodes| {
            nodes
                .iter()
                .find(|node| node["attributes"]["name"].as_str() == Some("csrf_token"))
        })
        .and_then(|node| node["attributes"]["value"].as_str())
        .ok_or("CSRF token not found in flow response")?
        .to_string();

    let mut all_cookies = Vec::new();
    if let Some(existing_cookie) = cookie {
        all_cookies.push(existing_cookie.to_string());
    }
    all_cookies.extend(flow_cookies);

    Ok(FlowResult {
        flow,
        csrf_token,
        cookies: all_cookies,
    })
}

pub async fn post_flow(
    client: &Client,
    public_url: &str,
    endpoint: &str,
    flow_id: &str,
    data: serde_json::Value,
    cookies: &[String],
) -> Result<PostFlowResult, Box<dyn Error>> {
    let cookie_header = cookies.join("; ");
    let url = format!("{}/self-service/{}?flow={}", public_url, endpoint, flow_id);
    let url = url.replace("localhost", "127.0.0.1");

    let response = client
        .post(&url)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::COOKIE, cookie_header)
        .json(&data)
        .send()
        .await
        .map_err(|e| format!("Failed to submit {} flow: {}", endpoint, e))?;

    let response_cookies: Vec<String> = response
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .collect();

    let status = response.status();
    if !status.is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("{} failed (status {}): {}", endpoint, status, error_text).into());
    }

    let data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse {} response: {}", endpoint, e))?;

    Ok(PostFlowResult {
        data,
        cookies: response_cookies,
    })
}

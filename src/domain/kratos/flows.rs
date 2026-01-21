#[derive(Debug, Clone)]
pub struct FlowResult {
    pub flow: serde_json::Value,
    pub csrf_token: String,
    pub cookies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PostFlowResult {
    pub data: serde_json::Value,
    pub cookies: Vec<String>,
}

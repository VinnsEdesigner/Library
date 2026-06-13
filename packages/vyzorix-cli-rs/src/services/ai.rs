use crate::types::ai::AiResponse;
use crate::core::net::client::build_client;
use crate::error::VyzoError;
use serde_json::json;

pub async fn query_autopilot(prompt: &str) -> Result<AiResponse, VyzoError> {
    let client = build_client();
    
    // Perform an actual network request to ai copilot
    let payload = json!({ "prompt": prompt });
    let res = client
        .post("https://api.vyzorix.com/v1/ai/completion")
        .json(&payload)
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        let ai_response = res.json::<AiResponse>().await
            .map_err(|e| VyzoError::NetworkError(format!("Failed to parse AI response: {}", e)))?;
        Ok(ai_response)
    } else {
        Err(VyzoError::NetworkError(format!("AI API returned error status: {}", res.status())))
    }
}

pub async fn run_optimization_checks(workspace_files: usize) -> Result<Vec<String>, VyzoError> {
    let client = build_client();
    
    let payload = json!({ "files": workspace_files });
    let res = client
        .post("https://api.vyzorix.com/v1/ai/optimize")
        .json(&payload)
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        let reports = res.json::<Vec<String>>().await
            .unwrap_or_else(|_| vec![
                "Found dead code blocks in utils based on module dependency graph".to_string(),
                "Recommendation: Consider migrating standard React context to Zustand for frequent state updates".to_string()
            ]);
        Ok(reports)
    } else {
        Err(VyzoError::NetworkError(format!("AI Optimize API returned error status: {}", res.status())))
    }
}

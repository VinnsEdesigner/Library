use crate::core::net::client::build_client;
use crate::error::VyzoError;
use crate::types::cicd::PipelineDefinition;

pub async fn validate_pipeline(engine: &str) -> Result<bool, VyzoError> {
    let client = build_client();
    
    let res = client
        .get(format!("https://api.vyzorix.com/v1/cicd/validate/{}", engine))
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        let def = res.json::<PipelineDefinition>().await
            .map_err(|e| VyzoError::NetworkError(format!("Failed to parse pipeline definition: {}", e)))?;
        Ok(def.valid)
    } else {
        Err(VyzoError::NetworkError(format!("API returned error status: {}", res.status())))
    }
}

pub async fn fetch_pipeline_templates(engine: &str) -> Result<std::collections::HashMap<String, String>, VyzoError> {
    let client = build_client();
    
    let res = client
        .get(format!("https://api.vyzorix.com/v1/cicd/templates/{}", engine))
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        let templates = res.json::<std::collections::HashMap<String, String>>().await
            .map_err(|e| VyzoError::NetworkError(format!("Failed to parse templates: {}", e)))?;
        Ok(templates)
    } else {
        Err(VyzoError::NetworkError(format!("API returned error status: {}", res.status())))
    }
}

use crate::types::infra::CloudState;
use crate::core::net::client::build_client;
use crate::error::VyzoError;

pub async fn fetch_cloud_state(provider: &str) -> Result<CloudState, VyzoError> {
    let client = build_client();
    
    let res = client
        .get(format!("https://api.vyzorix.com/v1/infra/state/{}", provider))
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        let state = res.json::<CloudState>().await
            .map_err(|e| VyzoError::NetworkError(format!("Failed to parse cloud state: {}", e)))?;
        Ok(state)
    } else {
        Err(VyzoError::NetworkError(format!("API returned error status: {}", res.status())))
    }
}

pub async fn provision_infrastructure(provider: &str) -> Result<Vec<String>, VyzoError> {
    let client = build_client();
    
    let res = client
        .post(format!("https://api.vyzorix.com/v1/infra/provision/{}", provider))
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        let logs = res.json::<Vec<String>>().await
            .unwrap_or_else(|_| vec!["Provisioning completed successfully.".to_string()]);
        Ok(logs)
    } else {
        Err(VyzoError::NetworkError(format!("API returned error status: {}", res.status())))
    }
}

pub async fn check_workspace_availability(name: &str) -> Result<bool, VyzoError> {
    let client = build_client();
    let res = client
        .get(format!("https://api.vyzorix.com/v1/workspace/check/{}", name))
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    Ok(res.status().is_success())
}

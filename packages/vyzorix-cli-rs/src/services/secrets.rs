use crate::core::net::client::build_client;
use crate::error::VyzoError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VyzoSecret {
    pub key: String,
    pub provider: String,
}

pub async fn list_secrets() -> Result<Vec<VyzoSecret>, VyzoError> {
    let client = build_client();
    
    let res = client
        .get("https://api.vyzorix.com/v1/secrets")
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        let secrets = res.json::<Vec<VyzoSecret>>().await
            .map_err(|e| VyzoError::NetworkError(format!("Failed to parse secrets list: {}", e)))?;
        Ok(secrets)
    } else {
        Err(VyzoError::NetworkError(format!("Secrets API returned error: {}", res.status())))
    }
}

use crate::core::net::client::build_client;
use crate::error::VyzoError;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ThemeConfig {
    pub files: HashMap<String, String>,
}

pub async fn fetch_theme_bundle() -> Result<ThemeConfig, VyzoError> {
    let client = build_client();
    
    let res = client
        .get("https://api.vyzorix.com/v1/theme/bundle")
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        let config = res.json::<ThemeConfig>().await
            .map_err(|e| VyzoError::NetworkError(format!("Failed to parse theme bundle: {}", e)))?;
        Ok(config)
    } else {
        Err(VyzoError::NetworkError(format!("Theme API returned error: {}", res.status())))
    }
}

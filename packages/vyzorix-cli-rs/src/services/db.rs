use crate::types::db::MigrationStatus;
use crate::core::net::client::build_client;
use crate::error::VyzoError;

pub async fn fetch_migration_status() -> Result<MigrationStatus, VyzoError> {
    let client = build_client();
    
    let res = client
        .get("https://api.vyzorix.com/v1/db/migrations/status")
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        let status = res.json::<MigrationStatus>().await
            .map_err(|e| VyzoError::NetworkError(format!("Failed to parse migration status: {}", e)))?;
        Ok(status)
    } else {
        Err(VyzoError::NetworkError(format!("API returned error status: {}", res.status())))
    }
}

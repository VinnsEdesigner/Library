use crate::core::net::client::build_client;
use crate::error::VyzoError;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct TelemetryAlert {
    pub severity: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct RealtimeStats {
    pub cpu_usage: f32,
    pub mem_usage_mb: u32,
    pub active_regions: u32,
    pub latest_log: String,
}

pub async fn push_metric(name: &str, value: f64) {
    let client = build_client();
    let payload = json!({ "metric": name, "value": value });
    
    let _ = client
        .post("https://api.vyzorix.com/v1/telemetry/push")
        .json(&payload)
        .send()
        .await;
    
    tracing::debug!("Metric emitted: {}={}", name, value);
}

pub async fn fetch_alerts() -> Result<Vec<TelemetryAlert>, VyzoError> {
    let client = build_client();
    let res = client.get("https://api.vyzorix.com/v1/telemetry/alerts").send().await.map_err(|e| VyzoError::NetworkError(e.to_string()))?;
    
    if res.status().is_success() {
        let alerts = res.json::<Vec<TelemetryAlert>>().await.map_err(|e| VyzoError::NetworkError(format!("Failed to parse alerts: {}", e)))?;
        Ok(alerts)
    } else {
        Err(VyzoError::NetworkError(format!("Alerts API returned error: {}", res.status())))
    }
}

pub async fn stream_realtime_stats() -> Result<impl tokio_stream::Stream<Item = Result<RealtimeStats, VyzoError>>, VyzoError> {
    use tokio_stream::StreamExt;
    use futures_util::StreamExt as _;

    let client = build_client();
    let res = client
        .get("https://api.vyzorix.com/v1/telemetry/realtime/stream")
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    Ok(res.bytes_stream().map(|b| {
        let b = b.map_err(|e| VyzoError::NetworkError(e.to_string()))?;
        serde_json::from_slice::<RealtimeStats>(&b)
            .map_err(|e| VyzoError::NetworkError(format!("Failed to parse realtime stats: {}", e)))
    }))
}

pub async fn set_threshold(metric: &str, value: f64) -> Result<(), VyzoError> {
    let client = build_client();
    let res = client
        .post("https://api.vyzorix.com/v1/telemetry/thresholds")
        .json(&json!({ "metric": metric, "value": value }))
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(VyzoError::NetworkError(format!("Failed to set threshold: {}", res.status())))
    }
}

pub async fn set_alert_threshold(key: &str, val: f64) -> Result<(), VyzoError> {
    let client = build_client();
    let res = client
        .post("https://api.vyzorix.com/v1/telemetry/thresholds")
        .json(&json!({ "metric": key, "threshold": val }))
        .send()
        .await
        .map_err(|e| VyzoError::NetworkError(e.to_string()))?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(VyzoError::NetworkError(format!("Threshold API returned error: {}", res.status())))
    }
}


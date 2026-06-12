use anyhow::Result;
use colored::*;
use crate::core::net::client::build_client;

#[derive(serde::Deserialize)]
struct TelemetryData {
    views: String,
    invokes: String,
    p99_latency_ms: String,
    bandwidth_gb: String,
}

pub async fn run(days: u32) -> Result<()> {
    println!("{}", format!("Compiling Workspace Analytics (T-{} Days)...", days).truecolor(225, 29, 72).bold());
    let client = build_client();
    let res = client.get(format!("https://api.vyzorix.com/v1/telemetry/report?days={}", days)).send().await;

    let data = match res {
        Ok(r) if r.status().is_success() => r.json::<TelemetryData>().await.map_err(|e| anyhow::anyhow!("Failed to parse telemetry: {}", e))?,
        _ => return Err(anyhow::anyhow!("Telemetry service currently unavailable.")),
    };
    
    println!("\n  Page Views:       {}", data.views.white().bold());
    println!("  API Invokes:      {}", data.invokes.white().bold());
    println!("  P99 Latency:      {} ms", data.p99_latency_ms.white().bold());
    println!("  Bandwidth Used:   {} GB", data.bandwidth_gb.white().bold());
    
    println!("\n{} Data aggregated successfully via telemetry core.", "✔".truecolor(225, 29, 72));
    Ok(())
}

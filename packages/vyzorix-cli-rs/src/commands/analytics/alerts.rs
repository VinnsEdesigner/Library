use anyhow::Result;
use colored::*;
use crate::services::telemetry;

pub async fn run() -> Result<()> {
    println!("{}", "Active Telemetry Alerts".truecolor(225, 29, 72).bold());
    
    match telemetry::fetch_alerts().await {
        Ok(alerts) => {
            if alerts.is_empty() {
                println!("\n  {}", "No active alerts in this environment.".green());
            } else {
                for alert in alerts {
                    let color = match alert.severity.as_str() {
                        "1" => Color::Red,
                        "2" => Color::Yellow,
                        _ => Color::White,
                    };
                    println!("  {} [SEV-{}] {}", "⚠".color(color).bold(), alert.severity, alert.message);
                }
            }
        }
        Err(e) => {
            println!("{} Failed to fetch active telemetry alerts: {}", "✖".red(), e);
        }
    }
    
    Ok(())
}

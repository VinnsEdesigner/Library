use anyhow::Result;
use crate::cli::AnalyticsAction;
use crate::services::telemetry;
use colored::*;

pub mod realtime;
pub mod report;
pub mod alerts;

pub async fn execute(action: AnalyticsAction) -> Result<()> {
    match action {
        AnalyticsAction::Realtime => realtime::run().await?,
        AnalyticsAction::Report { days } => report::run(days).await?,
        AnalyticsAction::Alerts => alerts::run().await?,
        AnalyticsAction::SetThreshold { key, val } => {
            println!("{}", format!("Updating threshold for {} to {}...", key, val).truecolor(225, 29, 72).bold());
            match telemetry::set_alert_threshold(&key, val).await {
                Ok(_) => println!("{} Threshold configured successfully.", "✔".truecolor(225, 29, 72)),
                Err(e) => println!("{} Failed to update threshold: {}", "✖".red(), e),
            }
        }
    }
    Ok(())
}


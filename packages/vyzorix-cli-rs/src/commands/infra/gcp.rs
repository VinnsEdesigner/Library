use anyhow::Result;
use colored::*;
use crate::services::infra;

pub async fn run() -> Result<()> {
    println!("{}", "Orchestrating GCP Environments...".truecolor(225, 29, 72).bold());
    match infra::provision_infrastructure("gcp").await {
        Ok(logs) => {
            for log in logs {
                println!("{} {}", "✔".white(), log);
            }
            println!("\n{} GCP Infrastructure aligned.", "✔".truecolor(225, 29, 72));
        }
        Err(e) => {
            println!("{} GCP Provisioning failed: {}", "✖".red(), e);
        }
    }
    Ok(())
}

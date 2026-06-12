use anyhow::Result;
use colored::*;
use crate::services::infra;

pub async fn run() -> Result<()> {
    println!("{}", "Orchestrating Azure Environments...".truecolor(225, 29, 72).bold());
    match infra::provision_infrastructure("azure").await {
        Ok(logs) => {
            for log in logs {
                println!("{} {}", "✔".white(), log);
            }
            println!("\n{} Azure Infrastructure aligned.", "✔".truecolor(225, 29, 72));
        }
        Err(e) => {
            println!("{} Azure Provisioning failed: {}", "✖".red(), e);
        }
    }
    Ok(())
}

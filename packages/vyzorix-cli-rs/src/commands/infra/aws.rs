use anyhow::Result;
use colored::*;
use crate::services::infra;

pub async fn run() -> Result<()> {
    println!("{}", "Orchestrating AWS Environments...".truecolor(225, 29, 72).bold());
    
    match infra::provision_infrastructure("aws").await {
        Ok(logs) => {
            for log in logs {
                println!("{} {}", "✔".white(), log);
            }
            println!("\n{} AWS Infrastructure aligned.", "✔".truecolor(225, 29, 72));
        }
        Err(e) => {
            println!("{} AWS Provisioning failed: {}", "✖".red(), e);
        }
    }
    
    Ok(())
}

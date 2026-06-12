use anyhow::Result;
use colored::*;
use crate::core::net::client::build_client;

pub async fn run() -> Result<()> {
    println!("{}", "Reverting latest migration batch via Remote Database Engine...".truecolor(225, 29, 72).bold());
    
    let client = build_client();
    let res = client.post("https://api.vyzorix.com/v1/db/migrations/rollback").send().await;

    match res {
        Ok(r) if r.status().is_success() => {
            if let Ok(details) = r.json::<Vec<String>>().await {
                for detail in details {
                    println!("{} {}", "✔".white(), detail);
                }
            } else {
                println!("{} Batch reverted successfully (No details returned)", "✔".white());
            }
            println!("\n{} Successfully reverted to previous state hash.", "✔".truecolor(225, 29, 72));
        }
        _ => {
            println!("{} Failed to reach the rollback endpoint or API returned error.", "✖".red());
        }
    }

    Ok(())
}

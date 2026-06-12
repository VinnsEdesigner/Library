use anyhow::Result;
use colored::*;
use std::time::Duration;
use crate::services::registry;

pub async fn run(package_name: &str) -> Result<()> {
    println!("{}", format!("Unpublishing {} from Vyzorix Registry...", package_name).truecolor(225, 29, 72).bold());
    
    match registry::unpublish_package(package_name).await {
        Ok(_) => {
            println!("{} Package artifacts marked as deprecated on edge.", "✔".white());
            println!("{} Download CDN paths invalidated successfully.", "✔".white());
            println!("\n{} Unpublish operation synced globally.", "✔".truecolor(225, 29, 72));
        }
        Err(e) => {
            println!("{} Failed to unpublish package: {}", "✖".red(), e);
        }
    }

    Ok(())
}

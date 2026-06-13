use anyhow::Result;
use colored::*;
use crate::services::registry;

pub async fn run(query: &str) -> Result<()> {
    println!("{}", format!("Searching Registry for '{}'...", query).truecolor(225, 29, 72).bold());
    
    match registry::list_packages(query).await {
        Ok(packages) => {
            if packages.is_empty() {
                println!("\n  {}", "No packages matched your query.".black().bright_black());
            } else {
                println!("");
                for package in &packages {
                    println!("  {} {} {}", "⚡".white(), package.name.white().bold(), package.version.black().bright_black());
                }
                println!("\n{} Total: {} packages matched.", "✔".truecolor(225, 29, 72), packages.len());
            }
        }
        Err(e) => {
            println!("{} Query for '{}' failed across registry: {}", "✖".red(), query, e);
        }
    }
    
    Ok(())
}

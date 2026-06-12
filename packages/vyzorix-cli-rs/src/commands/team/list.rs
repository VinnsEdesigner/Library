use anyhow::Result;
use colored::*;
use crate::services::team;

pub async fn run() -> Result<()> {
    println!("{}", "Fetching active team members...".truecolor(225, 29, 72).bold());

    match team::fetch_team_members().await {
        Ok(members) => {
            println!();
            for member in &members {
                let status_icon = if member.status.to_lowercase() == "active" { "●".green() } else { "○".yellow() };
                println!("  {} {} [{}]", status_icon, member.email.white().bold(), member.role.black().bright_black());
            }
            println!("\n{} Total: {} members mapped in directory.", "✔".truecolor(225, 29, 72), members.len());
        }
        Err(e) => {
            println!("{} Failed to fetch team directory: {}", "✖".red(), e);
        }
    }
    
    Ok(())
}

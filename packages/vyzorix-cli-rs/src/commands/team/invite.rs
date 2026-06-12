use crate::services::team;
use anyhow::Result;
use colored::*;
use dialoguer::{theme::SimpleTheme, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub async fn run(email: &str) -> Result<()> {
    println!("{}", format!("Inviting {} to the Vyzorix Workspace...", email).truecolor(225, 29, 72).bold());

    let roles = team::fetch_roles().await.unwrap_or_else(|_| vec!["admin".into(), "developer".into(), "viewer".into()]);
    let selection = Select::with_theme(&SimpleTheme)
        .with_prompt("Select a role for this user:")
        .default(1)
        .items(&roles)
        .interact()?;
    
    let selected_role = &roles[selection];

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.white} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    pb.set_message(format!("Provisioning IAM policies for {} [role: {}]...", email, selected_role));
    
    match team::invite_member(email, selected_role).await {
        Ok(_) => {
            pb.finish_with_message(format!("{} Secure invite sent to {} as {}", "✔".truecolor(225, 29, 72), email.white().bold(), selected_role.white().bold()));
        }
        Err(e) => {
            pb.finish_with_message(format!("{} Failed to send invite: {}", "✖".red(), e));
        }
    }
    
    Ok(())
}

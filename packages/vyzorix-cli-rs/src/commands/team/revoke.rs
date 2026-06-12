use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use crate::services::team;

pub async fn run(email: &str) -> Result<()> {
    println!("{}", format!("Revoking access for {}...", email).truecolor(225, 29, 72).bold());

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.white} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(90));

    pb.set_message("Invalidating active session tokens & purging privileges...");
    
    match team::revoke_member(email).await {
        Ok(_) => {
            pb.finish_with_message(format!("{} Access completely revoked for {}.", "✔".truecolor(225, 29, 72), email.white().bold()));
        }
        Err(e) => {
            pb.finish_with_message(format!("{} Failed to revoke access: {}", "✖".red(), e));
        }
    }
    
    Ok(())
}

use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use crate::services::auth;
use crate::cli::AuthAction;

pub async fn execute(action: AuthAction) -> Result<()> {
    match action {
        AuthAction::Logout => {
            println!("{}", "Revoking Vyzorix Cloud session...".truecolor(225, 29, 72).bold());
            match auth::logout_device().await {
                Ok(_) => println!("{} Session token formally invalidated.", "✔".truecolor(225, 29, 72)),
                Err(e) => println!("{} Failed to logout: {}", "✖".red(), e),
            }
        }
        AuthAction::Login { force } => {
            if force {
                println!("{}", "Force re-authenticating with Vyzorix Cloud...".truecolor(225, 29, 72).bold());
            } else {
                println!("{}", "Authenticating with Vyzorix Cloud...".truecolor(225, 29, 72).bold());
            }
            
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.white} {msg}")
                    .unwrap(),
            );
            pb.enable_steady_tick(Duration::from_millis(100));

            pb.set_message("Requesting authorization & exchanging PKCE code...");
            
            match auth::login_device("developer@vyzorix.com").await {
                Ok(_) => {
                    pb.finish_with_message(format!("{} Successfully authenticated as developer@vyzorix.com.", "✔".truecolor(225, 29, 72)));
                    println!("\n{}\n", "API tokens safely mounted in local keychain context.".white().bold());
                }
                Err(e) => {
                    pb.finish_with_message(format!("{} Authentication failed: {}", "✖".red(), e));
                }
            }
        }
        AuthAction::Status => {
            println!("{}", "Verifying Vyzorix Cloud Session Integrity...".truecolor(225, 29, 72).bold());
            
            match auth::check_session().await {
                Ok(status) => {
                    if status.authenticated {
                        println!("\n  {} Status:       {}", "●".green(), "Authenticated".white().bold());
                        println!("  {} Identity:     {}", "›".white(), status.user.unwrap_or_else(|| "Unknown".into()).white());
                        println!("  {} Workspace:    {}", "›".white(), status.organization.unwrap_or_else(|| "Default".into()).white());
                    } else {
                        println!("\n  {} Status:       {}", "○".yellow(), "Not Authenticated".white().bold());
                        println!("\n{} Run `vyzorix auth login` to start a new session.", "ℹ".blue());
                    }
                }
                Err(e) => {
                    println!("{} Session verification failed: {}", "✖".red(), e);
                }
            }
        }
    }
    
    Ok(())
}

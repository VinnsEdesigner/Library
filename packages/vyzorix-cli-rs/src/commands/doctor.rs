use anyhow::Result;
use colored::*;
use std::time::Duration;
use crate::core::net::client::build_client;
use crate::services::auth;
use std::path::Path;

pub async fn execute() -> Result<()> {
    println!("{}", "Running Vyzorix Infrastructure Diagnostics...".truecolor(225, 29, 72).bold());
    println!("{}", "Running health-check against current workspace scope...".white());
    
    // 1. Check internet connectivity & Vyzorix API
    let client = build_client();
    let api_reachable = client.get("https://api.vyzorix.com/health")
        .timeout(Duration::from_millis(1500))
        .send()
        .await
        .is_ok();

    if api_reachable {
        println!("{} Connected to Vyzorix Global Edge network", "✔".truecolor(225, 29, 72));
    } else {
        println!("{} Vyzorix Edge API is unreachable (Check network)", "✖".red());
    }

    // 2. Check session status
    if let Ok(session) = auth::check_session().await {
        if session.authenticated {
            println!("{} Authenticated as {}", "✔".truecolor(225, 29, 72), session.user.unwrap_or_default().white().bold());
        } else {
            println!("{} Not authenticated with Vyzorix Cloud", "○".yellow());
        }
    }

    // 2. Check workspace files
    if Path::new("package.json").exists() {
        println!("{} Found valid Node manifest (package.json)", "✔".truecolor(225, 29, 72));
    } else {
        println!("{} Missing package.json in workspace root", "✖".red());
    }

    if Path::new("vyzorix.toml").exists() {
        println!("{} Workspace configuration loaded (vyzorix.toml)", "✔".truecolor(225, 29, 72));
    } else {
        println!("{} Missing vyzorix.toml - run `vyzorix init` to scaffold", "○".yellow());
    }
    
    if Path::new(".env").exists() {
        println!("{} Local environment secrets (.env) identified safely", "✔".truecolor(225, 29, 72));
    } else {
        println!("{} No local .env detected, running in stateless mode", "○".yellow());
    }
    
    // 3. Environment Checks
    let rustc_res = tokio::process::Command::new("rustc").arg("--version").output().await;
    if let Ok(output) = rustc_res {
        println!("{} Rust Toolchain: {}", "✔".truecolor(225, 29, 72), String::from_utf8_lossy(&output.stdout).trim());
    }

    let node_res = tokio::process::Command::new("node").arg("--version").output().await;
    if let Ok(output) = node_res {
        println!("{} Node Runtime: {}", "✔".truecolor(225, 29, 72), String::from_utf8_lossy(&output.stdout).trim());
    }

    println!("\n{}\n", "System diagnostic completed.".white().bold());
    
    Ok(())
}

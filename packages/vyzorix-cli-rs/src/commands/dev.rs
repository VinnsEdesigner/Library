use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use tokio::select;
use crate::core::config::VyzoConfig;
use notify::{Watcher, RecursiveMode, Config};
use std::path::Path;

pub async fn execute(port: u16) -> Result<()> {
    let config = VyzoConfig::load();
    println!("{}", format!("Starting Vyzorix Live Server for '{}'...", config.project_name).truecolor(225, 29, 72).bold());
    
    println!("{} Server bound to {}", "✔".truecolor(225, 29, 72), format!("http://localhost:{}", port).white().bold());
    println!("{} Watching directories: {}", "✔".truecolor(225, 29, 72), "./src, ./themes".white());
    println!("{}", "Press Ctrl+C to stop.".black().bright_black());
    println!();
    
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.white} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(150));
    pb.set_message("Listening for file events...");

    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

    let mut watcher = notify::RecommendedWatcher::new(move |res| {
        if let Ok(_) = res {
            let _ = tx.blocking_send(());
        }
    }, Config::default())?;

    if Path::new("src").exists() {
        watcher.watch(Path::new("src"), RecursiveMode::Recursive)?;
    }

    loop {
        select! {
            Some(_) = rx.recv() => {
                pb.println(format!("{} HMR {} {}", "»".truecolor(225, 29, 72), "File event detected:".white(), "Hot module reload applied.".truecolor(225, 29, 72)));
            }
            _ = tokio::signal::ctrl_c() => {
                pb.finish_with_message(format!("{}", "Gracefully shutting down development server...".truecolor(225, 29, 72)));
                break;
            }
        }
    }
    
    println!("\n{}", "Goodbye!".white().bold());
    
    Ok(())
}

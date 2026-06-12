use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use crate::core::config::VyzoConfig;
use crate::core::state::VyzoState;
use crate::core::fs::archive;
use crate::services::{edge, auth};
use std::path::Path;

pub async fn execute(env: &str) -> Result<()> {
    // Check session first
    let session = auth::check_session().await?;
    if !session.authenticated {
        println!("{} You must be authenticated to deploy. Run `vyzorix auth login`.", "✖".red());
        return Ok(());
    }

    println!("{}", format!("Preparing deployment to Vyzorix Edge ({env})...").truecolor(225, 29, 72).bold());

    // Load Configuration context
    let config = VyzoConfig::load();
    println!("{} Read workspace context: {}", "✔".truecolor(225, 29, 72), config.project_name.white().bold());

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.white} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(80));

    pb.set_message("Bundling static assets and edge functions...");
    let bundle_path = ".vyzo/dist.tar.gz";
    if !Path::new(".vyzo").exists() {
        tokio::fs::create_dir_all(".vyzo").await?;
    }
    
    // Actually compress using the real archive module
    archive::bundle_workspace(bundle_path, "src").map_err(|e| anyhow::anyhow!("Bundling failed: {}", e))?;

    pb.set_message(format!("Uploading securely to Vyzorix Cloud target [{env}]..."));
    
    let metadata = std::fs::metadata(bundle_path)?;
    let total_size = metadata.len();
    
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.white} {msg} [{bar:40.truecolor(225, 29, 72)}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("━╾ "),
    );
    pb.set_length(total_size);

    let pb_cloned = pb.clone();
    match edge::deploy_bundle_with_progress(bundle_path, total_size, move |current| {
        pb_cloned.set_position(current);
    }).await {
        Ok(deployment) => {
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.white} {msg}")
                    .unwrap(),
            );
            pb.enable_steady_tick(Duration::from_millis(80));
            pb.set_message("Waiting for build runner...");
            
            use tokio_stream::StreamExt;
            match edge::stream_deployment_logs(&deployment.id).await {
                Ok(mut stream) => {
                    while let Some(log_res) = stream.next().await {
                        if let Ok(log) = log_res {
                            pb.println(format!("{} [{}] {}", "»".truecolor(225, 29, 72), log.step.white().bold(), log.message));
                            
                            if log.step == "COMPLETED" || log.step == "FAILED" {
                                break;
                            }
                        }
                    }
                }
                Err(_) => {
                    // Fallback to polling if stream fails
                    let mut seen_logs = 0;
                    loop {
                        if let Ok(logs) = edge::fetch_deployment_logs(&deployment.id).await {
                            for log in logs.iter().skip(seen_logs) {
                                pb.println(format!("{} [{}] {}", "»".truecolor(225, 29, 72), log.step.white().bold(), log.message));
                            }
                            seen_logs = logs.len();
                            if logs.iter().any(|l| l.step == "COMPLETED" || l.step == "FAILED") {
                                break;
                            }
                        }
                        tokio::time::sleep(Duration::from_millis(1000)).await;
                    }
                }
            }

            pb.finish_with_message(format!("{} Deployment synchronized! ({})", "✔".truecolor(225, 29, 72), deployment.id));
            
            // Save to state
            let mut state = VyzoState::load();
            state.last_deployment_id = Some(deployment.id.clone());
            let _ = state.save();
            
            println!("\n{} {}\n", "🚀 Live URL:".white().bold(), deployment.url.truecolor(225, 29, 72));
        }
        Err(e) => {
            pb.finish_with_message(format!("{} Deployment failed: {}", "✖".red(), e));
        }
    }


    Ok(())
}

use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use crate::core::fs::archive;
use crate::core::crypto::hash::compute_sha256;
use crate::services::registry;
use std::path::Path;

pub async fn run() -> Result<()> {
    println!("{}", "Publishing Workspace to Vyzorix Registry...".truecolor(225, 29, 72).bold());

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.white} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    pb.set_message("Packing artifacts & verifying source trees...");
    
    let publish_path = ".vyzo/registry.tar.gz";
    if !Path::new(".vyzo").exists() {
        tokio::fs::create_dir_all(".vyzo").await?;
    }
    
    archive::bundle_workspace(publish_path, "src").map_err(|e| anyhow::anyhow!("Packing failed: {}", e))?;

    pb.set_message("Calculating SHA256 integrity hash...");
    let package_data = tokio::fs::read(publish_path).await?;
    let integrity_hash = compute_sha256(&package_data);

    pb.set_message("Uploading payload to global edge nodes...");
    
    match registry::publish_package(publish_path, &integrity_hash).await {
        Ok(_) => {
            pb.finish_with_message(format!(
                "{} Package successfully published!\n  {}: {}", 
                "✔".truecolor(225, 29, 72),
                "Integrity Hash".white().bold(),
                integrity_hash.black().bright_black()
            ));
        }
        Err(e) => {
            pb.finish_with_message(format!("{} Publish operation failed: {}", "✖".red(), e));
        }
    }
    
    Ok(())
}

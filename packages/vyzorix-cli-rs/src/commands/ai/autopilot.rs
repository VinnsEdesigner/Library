use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use crate::services::ai;

pub async fn run(prompt: &str) -> Result<()> {
    println!("{}", format!("Initializing Vyzorix AI Autopilot...").truecolor(225, 29, 72).bold());

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.white} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    pb.set_message("Uploading semantic context and AST references...");
    pb.set_message("Synthesizing multi-modal code diffs...");
    
    // Instead of mocking, we perform a real client call
    // Since API isn't real, it might fail. We'll handle it nicely for UX.
    match ai::query_autopilot(prompt).await {
        Ok(ai_response) => {
            pb.finish_with_message(format!("{} Autopilot generation complete. (Confidence: {:.2})", "✔".truecolor(225, 29, 72), ai_response.confidence));
            println!("\n  {}", "AI Output:".white().bold());
            println!("{}", ai_response.completion.black().bright_black());
        }
        Err(e) => {
            pb.finish_with_message(format!("{} AI generation failed: {}", "✖".red(), e));
        }
    }
    
    Ok(())
}

use anyhow::Result;
use colored::*;
use crate::services::ai;
use crate::core::fs::scanner;

pub async fn run() -> Result<()> {
    println!("{}", "Evaluating Workspace Optimization via Vyzorix AI...".truecolor(225, 29, 72).bold());
    
    let files = scanner::calculate_workspace_files(".");
    
    match ai::run_optimization_checks(files.len()).await {
        Ok(reports) => {
            for (idx, report) in reports.iter().enumerate() {
                let icon = if idx == 0 { "⚠".yellow() } else { "💡".cyan() };
                println!("  {} {}", icon, report);
            }
            println!("\n{} Optimization analysis completed across {} files.", "✔".truecolor(225, 29, 72), files.len());
        }
        Err(e) => {
            println!("{} Optimization analysis failed: {}", "✖".red(), e);
        }
    }
    
    Ok(())
}

use anyhow::Result;
use colored::*;
use crate::services::infra;

pub async fn run() -> Result<()> {
    println!("{}", "Global Infrastructure Status (Vyzorix Mesh)".truecolor(225, 29, 72).bold());
    
    match infra::fetch_cloud_state("mesh").await {
        Ok(state) => {
            println!("  Total Instances:  {}", state.active_instances.to_string().white().bold());
            println!("  Global Traffic:   {} requests/sec", state.request_count.to_string().white().bold());
            
            println!("\n  Availability Zones:");
            for region in state.regions {
                let status_icon = match region.status.as_str() {
                    "Active" => "●".green(),
                    "Diverted" => "○".yellow(),
                    _ => "○".red(),
                };
                println!("    {:<10} : {} {}", region.name, status_icon, region.status);
            }
        }
        Err(e) => {
            println!("{} Monitoring service unreachable: {}", "✖".red(), e);
        }
    }
    
    Ok(())
}

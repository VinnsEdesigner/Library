use anyhow::Result;
use colored::*;
use crate::services::db;

pub async fn run() -> Result<()> {
    println!("{}", "Database Migration Status".truecolor(225, 29, 72).bold());
    
    match db::fetch_migration_status().await {
        Ok(status) => {
            println!("  Engine:       {}", status.engine.white().bold());
            println!("  Applied:      {}", status.applied_count.to_string().white().bold());
            println!("  Pending:      {}", status.pending_count.to_string().white().bold());
            
            if !status.last_migration.is_empty() {
                println!("\n  Last Applied: {}", status.last_migration.black().bright_black());
            }
        }
        Err(e) => {
            println!("{} Database engine unreachable: {}", "✖".red(), e);
        }
    }
    
    Ok(())
}

use anyhow::Result;
use colored::*;
use crate::core::db_engine;

pub async fn run() -> Result<()> {
    println!("{} Seeding database with defined local instances...", "›".white());
    
    match db_engine::seed_database().await {
        Ok(count) => {
            println!("{} Bulk seeding sequence complete ({} records inserted).", "✔".truecolor(225, 29, 72), count.to_string().white().bold());
        }
        Err(e) => {
            println!("{} Seed process aborted: {}", "✖".red(), e);
        }
    }
    
    Ok(())
}

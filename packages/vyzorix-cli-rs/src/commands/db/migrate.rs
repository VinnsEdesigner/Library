use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use crate::core::db_engine;

pub async fn run(dry_run: bool) -> Result<()> {
    if dry_run {
        println!("{}", "Simulating Schema Migrations (Dry Run)...".yellow().bold());
    } else {
        println!("{}", "Initiating Schema Migrations...".truecolor(225, 29, 72).bold());
    }

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.white} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    if dry_run {
        pb.set_message("Compiling DDL plan for preview...");
    } else {
        pb.set_message("Applying transactional DDL diffs...");
    }
    
    match db_engine::apply_migrations(dry_run).await {
        Ok(plan) => {
            if dry_run {
                pb.finish_with_message(format!("{} Migration plan generated.", "✔".yellow()));
                if let Some(sql) = plan.sql_preview {
                    println!("\n{}\n", "--- SQL PREVIEW ---".black().bright_black());
                    println!("{}", sql.truecolor(150, 150, 150));
                    println!("\n{}\n", "-------------------".black().bright_black());
                }
                println!("{} Run without {} to apply these changes.", "◌".white(), "--dry-run".bold());
            } else {
                pb.finish_with_message(format!("{} Database schema is up-to-date.", "✔".truecolor(225, 29, 72)));
                for id in plan.migration_ids {
                    println!("  {} Applied migration: {}", " →".black().bright_black(), id.white().bold());
                }
            }
        }
        Err(e) => {
            pb.finish_with_message(format!("{} Schema operation failed: {}", "✖".red(), e));
        }
    }
    Ok(())
}

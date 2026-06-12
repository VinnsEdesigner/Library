pub mod cli;
pub mod commands;
pub mod core;
pub mod error;
pub mod utils;
pub mod types;
pub mod services;

use clap::Parser;
use cli::VyzoCli;
use colored::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Parse CLI arguments
    let args = VyzoCli::parse();

    // 2. Initialize Environment (Dev vs Prod mode)
    let env_mode = core::env::determine_environment(&args);

    // 3. Setup Logger based on Environment and verbosity
    utils::logger::init(&env_mode, args.verbose);

    tracing::debug!("Vyzorix CLI booting up in {:?} mode", env_mode);
    
    if args.dev {
        println!("{}", "⚠️  Running in Development Mode".truecolor(225, 29, 72).bold());
    }

    // 4. Dispatch the subcommand execution
    match commands::handle_command(args.command).await {
        Ok(_) => {
            tracing::debug!("Command executed successfully");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("{} {}", "error:".truecolor(225, 29, 72).bold(), e);
            std::process::exit(1);
        }
    }
}

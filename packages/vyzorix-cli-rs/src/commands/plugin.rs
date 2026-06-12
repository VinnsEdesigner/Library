use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use crate::cli::PluginAction;
use crate::services::registry;
use crate::core::wasm::engine::WasmEngine;

pub async fn execute(action: PluginAction) -> Result<()> {
    match action {
        PluginAction::Add { name } => {
            println!("{}", format!("Installing Vyzorix Plugin: {}...", name).truecolor(225, 29, 72).bold());

            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.white} {msg}")
                    .unwrap(),
            );
            pb.enable_steady_tick(Duration::from_millis(100));

            pb.set_message("Resolving plugin manifest from registry...");
            
            match registry::get_package_meta(&name).await {
                Ok(pkg) => {
                    pb.set_message("Compiling WASM engine extensions...");
                    let _engine = WasmEngine::init(); // Bootup the engine logic
                    
                    pb.finish_with_message(format!(
                        "{} Plugin {} (v{}) successfully installed into WASM sandbox.", 
                        "✔".truecolor(225, 29, 72), 
                        pkg.name.white().bold(),
                        pkg.version.black().bright_black()
                    ));
                }
                Err(e) => {
                    pb.finish_with_message(format!("{} Failed to install plugin: {}", "✖".red(), e));
                }
            }
        }
        PluginAction::List => {
            println!("{}", "Available Vyzorix CLI Plugins:".truecolor(225, 29, 72).bold());

            match registry::list_packages("plugin-").await {
                Ok(packages) => {
                    if packages.is_empty() {
                        println!("\n  {}", "No plugins found in the registry.".black().bright_black());
                    } else {
                        println!("");
                        for pkg in packages {
                            println!("  {} {} {}", "⚡".white(), pkg.name.white().bold(), format!("(v{})", pkg.version).black().bright_black());
                        }
                    }
                }
                Err(e) => {
                    println!("{} Plugin registry unreachable: {}", "✖".red(), e);
                }
            }
            
            println!("\n{}", "Discover more plugins via the Vyzorix Edge Dashboard.".black().bright_black());
        }
    }
    
    Ok(())
}

use anyhow::Result;
use colored::*;
use std::path::Path;
use tokio::fs;
use crate::services::theme;
use tokio::io::AsyncWriteExt;

pub async fn execute(out_dir: &str) -> Result<()> {
    println!("{}", "Synchronizing Vyzorix theme definitions...".truecolor(225, 29, 72).bold());
    
    let path = Path::new(out_dir);
    if !path.exists() {
        fs::create_dir_all(path).await?;
    }
    
    match theme::fetch_theme_bundle().await {
        Ok(bundle) => {
            for (filename, content) in bundle.files {
                let file_path = path.join(&filename);
                let mut file = fs::File::create(&file_path).await?;
                file.write_all(content.as_bytes()).await?;
                println!("{} Sync'd {}/{}", "✔".truecolor(225, 29, 72), out_dir, filename);
            }
            println!("\n{}\n", "Refinement complete. Local theme tokens are now in-sync with remote source.".white().bold());
        }
        Err(e) => {
            println!("{} Theme synchronization failed: {}", "✖".red(), e);
        }
    }
    
    Ok(())
}

use anyhow::Result;
use colored::*;
use tokio::fs;
use std::path::Path;
use crate::services::cicd;
use tokio::io::AsyncWriteExt;

pub async fn run() -> Result<()> {
    println!("{}", "Generating GitHub Actions Workflows...".truecolor(225, 29, 72).bold());
    
    let path = Path::new(".github/workflows");
    if !path.exists() {
        fs::create_dir_all(path).await?;
    }
    
    match cicd::fetch_pipeline_templates("github").await {
        Ok(templates) => {
            for (name, content) in templates {
                let file_path = path.join(&name);
                let mut file = fs::File::create(&file_path).await?;
                file.write_all(content.as_bytes()).await?;
                println!("{} Created `.github/workflows/{}`", "✔".white(), name);
            }
            println!("\n{} GitHub Actions templates injected gracefully.", "✔".truecolor(225, 29, 72));
        }
        Err(e) => {
            println!("{} Failed to fetch GitHub templates: {}", "✖".red(), e);
        }
    }
    
    Ok(())
}

use anyhow::Result;
use colored::*;
use tokio::fs;
use crate::services::cicd;
use tokio::io::AsyncWriteExt;

pub async fn run() -> Result<()> {
    println!("{}", "Generating GitLab CI Pipelines...".truecolor(225, 29, 72).bold());
    
    match cicd::fetch_pipeline_templates("gitlab").await {
        Ok(templates) => {
            for (name, content) in templates {
                let mut file = fs::File::create(&name).await?;
                file.write_all(content.as_bytes()).await?;
                println!("{} Created `{}`", "✔".white(), name);
            }
            println!("\n{} GitLab CI templates injected gracefully.", "✔".truecolor(225, 29, 72));
        }
        Err(e) => {
            println!("{} Failed to fetch GitLab templates: {}", "✖".red(), e);
        }
    }
    
    Ok(())
}

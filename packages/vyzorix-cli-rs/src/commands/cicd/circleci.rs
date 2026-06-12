use anyhow::Result;
use colored::*;
use tokio::fs;
use std::path::Path;
use crate::services::cicd;
use tokio::io::AsyncWriteExt;

pub async fn run() -> Result<()> {
    println!("{}", "Generating CircleCI Configs...".truecolor(225, 29, 72).bold());
    
    let path = Path::new(".circleci");
    if !path.exists() {
        fs::create_dir_all(path).await?;
    }
    
    match cicd::fetch_pipeline_templates("circleci").await {
        Ok(templates) => {
            for (name, content) in templates {
                let file_path = path.join(&name);
                let mut file = fs::File::create(&file_path).await?;
                file.write_all(content.as_bytes()).await?;
                println!("{} Created `.circleci/{}`", "✔".white(), name);
            }
            println!("\n{} CircleCI templates injected gracefully.", "✔".truecolor(225, 29, 72));
        }
        Err(e) => {
            println!("{} Failed to fetch CircleCI templates: {}", "✖".red(), e);
        }
    }
    
    Ok(())
}

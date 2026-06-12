use anyhow::Result;
use colored::*;
use std::time::Duration;
use dialoguer::{theme::SimpleTheme, Select, Confirm, Input};
use indicatif::{ProgressBar, ProgressStyle};
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncWriteExt;
use serde_json::json;
use crate::services::registry;

pub async fn execute(force: bool) -> Result<()> {
    println!("{}", "Initializing Vyzorix Secure Environment...".truecolor(225, 29, 72).bold());
    
    let workspace_name: String = Input::with_theme(&SimpleTheme)
        .with_prompt("Enter workspace namespace")
        .default("my-vyzo-app".into())
        .interact_text()?;

    println!("{} Checking namespace availability on Vyzorix Edge...", "◌".white());
    match registry::check_namespace_availability(&workspace_name).await {
        Ok(available) if !available => {
            println!("{} Namespace '{}' is already taken or unavailable.", "✖".red(), workspace_name);
            return Err(anyhow::anyhow!("Namespace conflict"));
        }
        Err(e) => {
            println!("{} Warning: Could not verify namespace availability: {}", "⚠".yellow(), e);
        }
        _ => {
            println!("{} Namespace '{}' is available.", "✔".green(), workspace_name);
        }
    }
    
    if force {
        tracing::warn!("Force flag provided. Existing configurations may be overwritten.");
    } else {
        // Interactive UX Prompts
        let proceed = Confirm::with_theme(&SimpleTheme)
            .with_prompt("Do you want to scaffold a new Vyzorix workspace here?")
            .default(true)
            .interact()?;

        if !proceed {
            println!("{}", "Aborted process.".white());
            return Ok(());
        }
        
        let frameworks = &["React + Tailwind", "Next.js + Vyzorix UI", "Vanilla TS (API Only)", "Fetch from Registry..."];
        let selection = Select::with_theme(&SimpleTheme)
            .with_prompt("Select a framework to inject:")
            .default(0)
            .items(&frameworks[..])
            .interact()?;
            
        let mut template_name = frameworks[selection].to_string();
        if selection == 3 {
             template_name = Input::with_theme(&SimpleTheme)
                .with_prompt("Enter template name (e.g., 'next-auth-postgres')")
                .interact_text()?;
        }

        println!("Target Template: {}\n", template_name.white().bold());
    }


    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.white} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(80));

    pb.set_message("Scaffolding directories...");
    create_dir_all("src/app").await?;
    create_dir_all("src/components").await?;
    create_dir_all(".vyzo").await?;
    
    pb.set_message("Writing package.json...");
    let mut pkg = File::create("package.json").await?;
    let pkg_json = json!({
        "name": workspace_name,
        "version": "1.0.0",
        "private": true,
        "vyzorix": {
            "template": template_name
        }
    });
    pkg.write_all(serde_json::to_string_pretty(&pkg_json).unwrap().as_bytes()).await?;

    pb.set_message("Provisioning vyzorix.toml...");
    let mut toml = File::create("vyzorix.toml").await?;
    let toml_content = format!(
r#"[workspace]
name = "{}"
env = "local"
template = "{}"
"#,
        workspace_name, template_name
    );
    toml.write_all(toml_content.as_bytes()).await?;

    pb.set_message(format!("Fetching remote template {}...", template_name));
    
    match registry::download_template(&template_name.to_lowercase().replace(" + ", "-")).await {
        Ok(files) => {
            for (path, content) in files {
                if let Some(parent) = std::path::Path::new(&path).parent() {
                    create_dir_all(parent).await?;
                }
                let mut f = File::create(&path).await?;
                f.write_all(content.as_bytes()).await?;
            }
        }
        Err(_) => {
            pb.set_message("Falling back to local fallback scaffolding...");
            match selection {
                0 => { // React + Tailwind
                    let mut index = File::create("src/main.tsx").await?;
                    index.write_all(br#"import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)"#).await?;
                    let mut app = File::create("src/App.tsx").await?;
                    app.write_all(br#"export default function App() {
  return (
    <div className="min-h-screen bg-slate-950 text-white flex items-center justify-center">
      <h1 className="text-4xl font-bold tracking-tight">Vyzorix + React</h1>
    </div>
  )
}"#).await?;
                },
                1 => { // Next.js + Vyzorix UI
                    let mut layout = File::create("src/app/layout.tsx").await?;
                    layout.write_all(br#"export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body className="antialiased bg-black text-white">{children}</body>
    </html>
  )
}"#).await?;
                    let mut page = File::create("src/app/page.tsx").await?;
                    page.write_all(br#"export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24">
      <div className="z-10 max-w-5xl w-full items-center justify-between font-mono text-sm flex">
        <p className="fixed left-0 top-0 flex w-full justify-center border-b border-gray-300 bg-gradient-to-b from-zinc-200 pb-6 pt-8 backdrop-blur-2xl dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit lg:static lg:w-auto  lg:rounded-xl lg:border lg:bg-gray-200 lg:p-4 lg:dark:bg-zinc-800/30">
          Vyzorix Edge Deployment
        </p>
      </div>
    </main>
  )
}"#).await?;
                },
                _ => { // Vanilla TS
                    let mut index = File::create("src/index.ts").await?;
                    index.write_all(br#"console.log("Vyzorix API Service Started");
export const handler = async (event: any) => {
  return { status: "ok", timestamp: new Date().toISOString() };
};"#).await?;
                }
            }
        }
    }

    pb.finish_with_message(format!("{} Scaffolding complete!", "✔".truecolor(225, 29, 72)));
    
    println!("\n{}\n", "Workspace ready. Secure operations may commence.".white().bold());
    Ok(())
}

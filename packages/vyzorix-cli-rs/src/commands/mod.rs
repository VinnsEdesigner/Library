pub mod init;
pub mod theme;
pub mod doctor;
pub mod auth;
pub mod deploy;
pub mod dev;
pub mod upgrade;
pub mod secrets;
pub mod plugin;
pub mod db;
pub mod infra;
pub mod analytics;
pub mod cicd;
pub mod ai;
pub mod team;
pub mod registry;

use crate::cli::Commands;

pub async fn handle_command(command: Commands) -> anyhow::Result<()> {
    match command {
        Commands::Init { force } => init::execute(force).await,
        Commands::Theme { out_dir } => theme::execute(&out_dir).await,
        Commands::Doctor => doctor::execute().await,
        Commands::Auth { action } => auth::execute(action).await,
        Commands::Deploy { env } => deploy::execute(&env).await,
        Commands::Dev { port } => dev::execute(port).await,
        Commands::Upgrade { force } => upgrade::execute(force).await,
        Commands::Secrets { action } => secrets::execute(action).await,
        Commands::Plugin { action } => plugin::execute(action).await,
        Commands::Db { action } => db::execute(action).await,
        Commands::Infra { action } => infra::execute(action).await,
        Commands::Analytics { action } => analytics::execute(action).await,
        Commands::Cicd { action } => cicd::execute(action).await,
        Commands::Ai { action } => ai::execute(action).await,
        Commands::Team { action } => team::execute(action).await,
        Commands::Registry { action } => registry::execute(action).await,
    }
}

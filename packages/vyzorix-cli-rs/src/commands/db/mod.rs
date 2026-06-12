use anyhow::Result;
use crate::cli::DbAction;

pub mod migrate;
pub mod seed;
pub mod rollback;
pub mod status;

pub async fn execute(action: DbAction) -> Result<()> {
    match action {
        DbAction::Migrate { dry_run } => migrate::run(dry_run).await?,
        DbAction::Seed => seed::run().await?,
        DbAction::Rollback => rollback::run().await?,
        DbAction::Status => status::run().await?,
    }
    Ok(())
}

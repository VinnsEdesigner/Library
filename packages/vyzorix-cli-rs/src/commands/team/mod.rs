use anyhow::Result;
use crate::cli::TeamAction;

pub mod invite;
pub mod list;
pub mod revoke;

pub async fn execute(action: TeamAction) -> Result<()> {
    match action {
        TeamAction::Invite { email } => invite::run(&email).await?,
        TeamAction::List => list::run().await?,
        TeamAction::Revoke { email } => revoke::run(&email).await?,
    }
    Ok(())
}

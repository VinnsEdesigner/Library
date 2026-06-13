use anyhow::Result;
use crate::cli::CicdAction;

pub mod github;
pub mod gitlab;
pub mod circleci;

pub async fn execute(action: CicdAction) -> Result<()> {
    match action {
        CicdAction::Github => github::run().await?,
        CicdAction::Gitlab => gitlab::run().await?,
        CicdAction::Circleci => circleci::run().await?,
    }
    Ok(())
}

use anyhow::Result;
use crate::cli::RegistryAction;

pub mod publish;
pub mod unpublish;
pub mod search;

pub async fn execute(action: RegistryAction) -> Result<()> {
    match action {
        RegistryAction::Publish => publish::run().await?,
        RegistryAction::Unpublish { package_name } => unpublish::run(&package_name).await?,
        RegistryAction::Search { query } => search::run(&query).await?,
    }
    Ok(())
}

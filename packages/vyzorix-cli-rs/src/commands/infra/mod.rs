use anyhow::Result;
use crate::cli::InfraAction;

pub mod aws;
pub mod gcp;
pub mod azure;
pub mod status;

pub async fn execute(action: InfraAction) -> Result<()> {
    match action {
        InfraAction::Aws => aws::run().await?,
        InfraAction::Gcp => gcp::run().await?,
        InfraAction::Azure => azure::run().await?,
        InfraAction::Status => status::run().await?,
    }
    Ok(())
}

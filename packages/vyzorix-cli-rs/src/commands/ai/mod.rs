use anyhow::Result;
use crate::cli::AiAction;

pub mod autopilot;
pub mod optimize;
pub mod audit;

pub async fn execute(action: AiAction) -> Result<()> {
    match action {
        AiAction::Autopilot { prompt } => autopilot::run(&prompt).await?,
        AiAction::Optimize => optimize::run().await?,
        AiAction::Audit => audit::run().await?,
    }
    Ok(())
}

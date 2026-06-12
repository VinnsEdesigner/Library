use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineDefinition {
    pub engine: String,
    pub valid: bool,
}

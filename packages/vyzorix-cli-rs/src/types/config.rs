use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceEnv {
    pub is_prod: bool,
    pub strict_mode: bool,
}

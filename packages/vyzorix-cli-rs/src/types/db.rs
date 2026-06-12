use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MigrationStatus {
    pub pending: u32,
    pub applied: u32,
}

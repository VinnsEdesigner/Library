use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CliContext {
    pub raw_args: Vec<String>,
    pub timestamp: i64,
}

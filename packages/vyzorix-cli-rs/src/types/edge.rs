use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EdgeDeployment {
    pub id: String,
    pub url: String,
    pub status: String,
}

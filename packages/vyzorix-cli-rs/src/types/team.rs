use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamMember {
    pub email: String,
    pub role: String,
    pub status: String,
}

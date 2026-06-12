use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionStatus {
    pub name: String,
    pub status: String, // "Active", "Diverted", "Offline"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudState {
    pub provider: String,
    pub healthy: bool,
    pub active_instances: u32,
    pub request_count: u64,
    pub regions: Vec<RegionStatus>,
}

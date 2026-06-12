use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AiResponse {
    pub completion: String,
    pub confidence: f64,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: String,
    pub data: String,
    pub priority: String, // "high", "medium", "low"
    pub status: String,   // "queued", "processing", "complete"
    pub timestamp: u64,
}

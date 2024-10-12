use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct QueueMessageRequest {
    pub queue: String,
    pub message: String,
}
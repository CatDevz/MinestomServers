use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiError {
    pub error: String,
    pub description: String,
}

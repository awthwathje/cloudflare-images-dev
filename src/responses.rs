use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub result: ImageResult,
    pub success: bool,
    pub errors: Vec<String>,
    pub messages: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ImageResult {
    pub id: String,
    pub filename: String,
    pub uploaded: DateTime<Utc>,
    #[serde(rename = "requireSignedURLs")]
    pub require_signed_urls: bool,
    pub variants: Vec<String>,
}

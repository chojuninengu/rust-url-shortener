use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShortenRequest {
    pub original_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShortenResponse {
    pub short_url: String,
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    #[serde(rename = "name")]
    pub code: String,
    #[serde(rename = "description")]
    pub description: String,
}

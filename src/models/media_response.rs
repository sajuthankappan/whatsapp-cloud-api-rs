use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaResponse {
    pub messaging_product: String,
    pub url: String,
    pub mime_type: String,
    pub sha256: String,
    pub file_size: i32,
    pub id: String,
}

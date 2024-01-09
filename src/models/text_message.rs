use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Text {
    body: String,
    preview_url: Option<bool>,
}

impl Text {
    pub fn new(body: &str) -> Self {
        Self {
            body: body.into(),
            preview_url: None,
        }
    }

    pub fn with_preview_url(body: &str) -> Self {
        Self {
            body: body.into(),
            preview_url: Some(true),
        }
    }
}

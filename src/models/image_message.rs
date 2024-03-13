use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    link: Option<String>,
    id: Option<String>,
    caption: Option<String>,
}

impl Image {
    pub fn new(link: &str, caption: Option<String>) -> Self {
        Self {
            link: Some(link.into()),
            id: None,
            caption,
        }
    }

    pub fn for_id(id: &str, caption: Option<String>) -> Self {
        Self {
            link: None,
            id: Some(id.into()),
            caption,
        }
    }
}

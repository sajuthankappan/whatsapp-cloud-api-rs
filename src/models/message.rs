use serde::{Deserialize, Serialize};

use super::Template;

const WHATSAPP: &str = "whatsapp";
const TEXT: &str = "text";
const TEMPLATE: &str = "template";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub to: String,
    pub messaging_product: String,
    pub recipient_type: Option<String>,
    #[serde(rename = "type")]
    pub message_type: String,
    pub text: Option<Text>,
    pub template: Option<Template>,
}

impl Message {
    pub fn from_text(to: &str, text: Text) -> Self {
        Self {
            to: to.into(),
            messaging_product: WHATSAPP.into(),
            recipient_type: None,
            message_type: TEXT.into(),
            text: Some(text),
            template: None,
        }
    }

    pub fn from_template(to: &str, template: Template) -> Self {
        Self {
            messaging_product: WHATSAPP.into(),
            recipient_type: None,
            message_type: TEMPLATE.into(),
            to: to.into(),
            text: None,
            template: Some(template),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Text {
    pub body: String,
    pub preview_url: Option<bool>,
}

impl Text {
    pub fn new(body: &str) -> Text {
        Self {
            body: body.into(),
            preview_url: None,
        }
    }

    pub fn with_preview_url(body: &str, preview_url: bool) -> Self {
        Self {
            body: body.into(),
            preview_url: Some(preview_url),
        }
    }
}

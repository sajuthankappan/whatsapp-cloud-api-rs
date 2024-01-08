use serde::{Deserialize, Serialize};

use crate::WHATSAPP;

use super::Context;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextMessage {
    to: String,
    messaging_product: String,
    recipient_type: Option<String>,
    text: Text,
    context: Option<Context>,
}

impl TextMessage {
    pub fn new(to: &str, text: Text, context: Option<Context>) -> TextMessage {
        Self {
            to: to.into(),
            messaging_product: WHATSAPP.into(),
            recipient_type: None,
            text,
            context,
        }
    }
}

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

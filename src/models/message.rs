use serde::{Deserialize, Serialize};

use crate::WHATSAPP;

use super::{template_message::Template, text_message::Text};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    to: String,
    messaging_product: String,
    recipient_type: Option<String>,
    context: Option<Context>,

    #[serde(rename = "type")]
    message_type: MessageType,
    text: Option<Text>,
    template: Option<Template>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Text,
    Template,
}

impl Message {
    pub fn from_text(to: &str, text: Text, context: Option<Context>) -> Self {
        Self {
            to: to.into(),
            messaging_product: WHATSAPP.into(),
            recipient_type: None,
            context,
            message_type: MessageType::Text,
            text: Some(text),
            template: None,
        }
    }

    pub fn from_template(to: &str, template: Template, context: Option<Context>) -> Self {
        Self {
            to: to.into(),
            messaging_product: WHATSAPP.into(),
            recipient_type: None,
            context,
            message_type: MessageType::Template,
            text: None,
            template: Some(template),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    pub message_id: String,
}

impl Context {
    pub fn new(message_id: &str) -> Self {
        Self {
            message_id: message_id.into(),
        }
    }
}

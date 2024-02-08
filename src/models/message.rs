use serde::{Deserialize, Serialize};

use crate::WHATSAPP;

use super::{template_message::Template, text_message::Text};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    biz_opaque_callback_data: Option<String>,
    context: Option<Context>,
    messaging_product: String,
    recipient_type: Option<String>,
    status: Option<StatusCode>,

    #[serde(rename = "type")]
    message_type: Option<MessageType>,
    template: Option<Template>,
    text: Option<Text>,
    to: String,
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
            biz_opaque_callback_data: None,
            context,
            messaging_product: WHATSAPP.into(),
            recipient_type: None,
            status: None,
            message_type: Some(MessageType::Text),
            template: None,
            text: Some(text),
            to: to.into(),
        }
    }

    pub fn from_template(to: &str, template: Template, context: Option<Context>) -> Self {
        Self {
            biz_opaque_callback_data: None,
            context,
            messaging_product: WHATSAPP.into(),
            recipient_type: None,
            status: None,
            message_type: Some(MessageType::Template),
            template: Some(template),
            text: None,
            to: to.into(),
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

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum StatusCode {
    Delivered,
    Read,
    Sent,
    Failed,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MessageStatus {
    messaging_product: String,
    message_id: String,
    status: StatusCode,
}

impl MessageStatus {
    pub fn for_read(message_id: &str) -> Self {
        Self {
            messaging_product: WHATSAPP.into(),
            message_id: message_id.into(),
            status: StatusCode::Read,
        }
    }
}

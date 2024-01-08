use serde::{Deserialize, Serialize};

use super::{
    template_message::{Template, TemplateMessage},
    text_message::{Text, TextMessage},
};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Message {
    Text(TextMessage),
    Template(TemplateMessage),
}

impl Message {
    pub fn from_text(to: &str, text: Text) -> Self {
        let text_message = TextMessage::new(to, text, None);
        Self::Text(text_message)
    }

    pub fn from_text_with_context(to: &str, text: Text, context: Context) -> Self {
        let text_message = TextMessage::new(to, text, Some(context));
        Self::Text(text_message)
    }

    pub fn from_template(to: &str, template: Template) -> Self {
        let template_message = TemplateMessage::new(to, template, None);
        Self::Template(template_message)
    }

    pub fn from_template_with_context(to: &str, template: Template, context: Context) -> Self {
        let template_message = TemplateMessage::new(to, template, Some(context));
        Self::Template(template_message)
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

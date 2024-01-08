use serde::{Deserialize, Serialize};

use crate::WHATSAPP;

use super::{component::Component, Context};

const DETERMINISTIC: &str = "deterministic";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TemplateMessage {
    to: String,
    messaging_product: String,
    recipient_type: Option<String>,
    template: Template,
    context: Option<Context>,
}

impl TemplateMessage {
    pub fn new(to: &str, template: Template, context: Option<Context>) -> Self {
        Self {
            to: to.into(),
            messaging_product: WHATSAPP.into(),
            recipient_type: None,
            template,
            context,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Template {
    name: String,
    language: Language,
    components: Option<Vec<Component>>,
}

impl Template {
    pub fn new(name: &str, language: &str) -> Self {
        let language = Language::new(language);

        Self {
            name: name.into(),
            language,
            components: None,
        }
    }

    pub fn with_components(name: &str, language: &str, components: Vec<Component>) -> Self {
        let language = Language::new(language);

        Self {
            name: name.into(),
            language,
            components: Some(components),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Language {
    pub policy: String,
    pub code: String,
}

impl Language {
    pub fn new(code: &str) -> Language {
        Self {
            policy: DETERMINISTIC.into(),
            code: code.into(),
        }
    }
}

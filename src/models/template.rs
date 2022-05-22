use serde::{Deserialize, Serialize};

use super::component::Component;

const DETERMINISTIC: &str = "deterministic";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub language: Language,
    pub components: Option<Vec<Component>>,
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

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Component {
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    pub sub_type: Option<ComponentSubType>,
    pub parameters: Option<Vec<Parameter>>,
    pub index: Option<u8>,
}

impl Component {
    pub fn new(component_type: ComponentType) -> Self {
        Self {
            component_type,
            sub_type: None,
            parameters: None,
            index: None,
        }
    }

    pub fn with_parameters(component_type: ComponentType, parameters: Vec<Parameter>) -> Self {
        Self {
            component_type,
            sub_type: None,
            parameters: Some(parameters),
            index: None,
        }
    }

    pub fn for_button(
        component_type: ComponentType,
        sub_type: ComponentSubType,
        parameters: Vec<Parameter>,
        index: u8,
    ) -> Self {
        Self {
            component_type,
            sub_type: Some(sub_type),
            parameters: Some(parameters),
            index: Some(index),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ComponentType {
    Header,
    Body,
    Button,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ComponentSubType {
    QuickReply,
    Url,
    Catalog,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Parameter {
    #[serde(rename = "type")]
    pub parameter_type: ParameterType,
    pub text: Option<String>,
    pub currency: Option<Currency>,
    pub date_time: Option<String>,
    pub image: Option<Media>,
    pub document: Option<Media>,
    pub video: Option<Media>,
}

impl Parameter {
    pub fn from_text(text: &str) -> Self {
        Self {
            parameter_type: ParameterType::Text,
            text: Some(text.into()),
            currency: None,
            date_time: None,
            image: None,
            document: None,
            video: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ParameterType {
    Currency,
    DateTime,
    Document,
    Image,
    Text,
    Video,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Currency {
    pub fallback_value: String,
    pub code: String,
    pub amount_1000: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DateTime {
    pub fallback_value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Media {
    pub id: Option<String>,
    pub link: Option<String>,
    pub caption: Option<String>,
    pub filename: Option<String>,
}

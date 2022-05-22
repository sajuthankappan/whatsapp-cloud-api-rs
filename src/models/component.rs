use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Component {
    #[serde(rename = "type")]
    pub component_type: String,
    pub sub_type: Option<String>,
    pub parameters: Option<Vec<Parameter>>,
}

impl Component {
    pub fn new(component_type: &str) -> Self {
        Self {
            component_type: component_type.into(),
            sub_type: None,
            parameters: None,
        }
    }

    pub fn with_parameters(component_type: &str, parameters: Vec<Parameter>) -> Self {
        Self {
            component_type: component_type.into(),
            sub_type: None,
            parameters: Some(parameters),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Parameter {
    #[serde(rename = "type")]
    pub parameter_type: String,
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
            parameter_type: "text".into(),
            text: Some(text.into()),
            currency: None,
            date_time: None,
            image: None,
            document: None,
            video: None,
        }
    }
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

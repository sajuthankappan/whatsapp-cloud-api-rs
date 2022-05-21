use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Component {
    #[serde(rename = "type")]
    pub component_type: String,
    pub sub_type: Option<String>,
    pub parameters: Option<String>,
}

impl Component {
    pub fn new(component_type: &str) -> Self {
        Component {
            component_type: component_type.into(),
            sub_type: None,
            parameters: None,
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

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Interactive {
    action: InteractiveAction,
    body: Option<InteractiveBody>,

    #[serde(rename = "type")]
    interactive_type: InteractiveType,
}

impl Interactive {
    pub fn for_button(buttons: Vec<InteractiveActionButton>, body_text: &str) -> Self {
        Self {
            action: InteractiveAction::new_buttons(buttons),
            interactive_type: InteractiveType::Button,
            body: Some(InteractiveBody::new(body_text)),
        }
    }

    pub fn for_list(button: &str, body_text: &str) -> Self {
        Self {
            action: InteractiveAction::new_list(button),
            interactive_type: InteractiveType::List,
            body: Some(InteractiveBody::new(body_text)),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InteractiveAction {
    button: Option<String>,
    buttons: Option<Vec<InteractiveActionButton>>,
    catalog_id: Option<String>,
    product_retailer_id: Option<String>,
    // TODO: Other fields
}

impl InteractiveAction {
    pub fn new_buttons(buttons: Vec<InteractiveActionButton>) -> Self {
        InteractiveAction {
            button: None,
            buttons: Some(buttons),
            catalog_id: None,
            product_retailer_id: None,
        }
    }

    pub fn new_list(button: &str) -> Self {
        InteractiveAction {
            button: Some(button.into()),
            buttons: None,
            catalog_id: None,
            product_retailer_id: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InteractiveBody {
    text: String,
}

impl InteractiveBody {
    fn new(text: &str) -> Self {
        Self { text: text.into() }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InteractiveActionButton {
    #[serde(rename = "type")]
    action_type: InteractiveActionButtonType,

    reply: InteractiveActionButtonReply,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InteractiveActionButtonReply {
    title: String,
    id: String,
}

impl InteractiveActionButton {
    pub fn new(title: &str, id: &str) -> Self {
        Self {
            action_type: InteractiveActionButtonType::Reply,
            reply: InteractiveActionButtonReply {
                title: title.into(),
                id: id.into(),
            },
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum InteractiveActionButtonType {
    Reply,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum InteractiveType {
    Button,
    CatalogMessage,
    List,
    Product,
    ProductList,
    Flow,
}

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

    pub fn for_list(
        button: &str,
        sections: Vec<InteractiveActionSection>,
        body_text: &str,
    ) -> Self {
        Self {
            action: InteractiveAction::new_list(button, sections),
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
    sections: Option<Vec<InteractiveActionSection>>,
    // TODO: Other fields
}

impl InteractiveAction {
    pub fn new_buttons(buttons: Vec<InteractiveActionButton>) -> Self {
        InteractiveAction {
            button: None,
            buttons: Some(buttons),
            catalog_id: None,
            product_retailer_id: None,
            sections: None,
        }
    }

    pub fn new_list(button: &str, sections: Vec<InteractiveActionSection>) -> Self {
        InteractiveAction {
            button: Some(button.into()),
            buttons: None,
            catalog_id: None,
            product_retailer_id: None,
            sections: Some(sections),
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InteractiveActionSection {
    // TODO: product_items
    rows: Vec<InteractiveActionSectionRow>,
    title: Option<String>,
}

impl InteractiveActionSection {
    pub fn new(rows: Vec<InteractiveActionSectionRow>) -> Self {
        Self { rows, title: None }
    }

    pub fn with_title(rows: Vec<InteractiveActionSectionRow>, title: &str) -> Self {
        Self {
            rows,
            title: Some(title.into()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InteractiveActionSectionRow {
    id: String,
    title: String,
    description: Option<String>,
}

impl InteractiveActionSectionRow {
    pub fn new(id: &str, title: &str) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
        }
    }

    pub fn with_description(id: &str, title: &str, description: &str) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: Some(description.into()),
        }
    }
}

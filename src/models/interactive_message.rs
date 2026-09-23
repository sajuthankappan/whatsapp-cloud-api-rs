use serde::{Deserialize, Serialize};

use super::component::Media;

const FLOW_MESSAGE_VERSION: &str = "3";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Interactive {
    action: InteractiveAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<InteractiveBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    footer: Option<InteractiveFooter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<InteractiveHeader>,

    #[serde(rename = "type")]
    interactive_type: InteractiveType,
}

impl Interactive {
    fn new(interactive_type: InteractiveType, action: InteractiveAction, body_text: &str) -> Self {
        Self {
            action,
            body: Some(InteractiveBody::new(body_text)),
            footer: None,
            header: None,
            interactive_type,
        }
    }

    pub fn for_button(buttons: Vec<InteractiveActionButton>, body_text: &str) -> Self {
        Self::new(
            InteractiveType::Button,
            InteractiveAction::new_buttons(buttons),
            body_text,
        )
    }

    pub fn for_list(
        button: &str,
        sections: Vec<InteractiveActionSection>,
        body_text: &str,
    ) -> Self {
        Self::new(
            InteractiveType::List,
            InteractiveAction::new_list(button, sections),
            body_text,
        )
    }

    /// Button that opens `url`
    pub fn for_cta_url(display_text: &str, url: &str, body_text: &str) -> Self {
        let parameters = InteractiveActionParameters {
            display_text: Some(display_text.into()),
            url: Some(url.into()),
            ..Default::default()
        };
        Self::new(
            InteractiveType::CtaUrl,
            InteractiveAction::named("cta_url", Some(parameters)),
            body_text,
        )
    }

    /// Button that opens a WhatsApp Flow
    pub fn for_flow(flow: FlowAction, body_text: &str) -> Self {
        Self::new(
            InteractiveType::Flow,
            InteractiveAction::named("flow", Some(flow.into_parameters())),
            body_text,
        )
    }

    /// Single product from a catalog. Body is optional (see [`Interactive::with_body`]); a header is not allowed.
    pub fn for_product(catalog_id: &str, product_retailer_id: &str) -> Self {
        let action = InteractiveAction {
            catalog_id: Some(catalog_id.into()),
            product_retailer_id: Some(product_retailer_id.into()),
            ..Default::default()
        };
        Self {
            action,
            body: None,
            footer: None,
            header: None,
            interactive_type: InteractiveType::Product,
        }
    }

    /// Up to 30 products from a catalog, grouped into sections. Only a text header is allowed, and it is required.
    pub fn for_product_list(
        header_text: &str,
        catalog_id: &str,
        sections: Vec<ProductSection>,
        body_text: &str,
    ) -> Self {
        let action = InteractiveAction {
            catalog_id: Some(catalog_id.into()),
            sections: Some(sections.into_iter().map(Into::into).collect()),
            ..Default::default()
        };
        Self::new(InteractiveType::ProductList, action, body_text)
            .with_header(InteractiveHeader::text(header_text))
    }

    /// Button that opens the business's catalog. Without a thumbnail product, the first catalog item's image is used.
    pub fn for_catalog(body_text: &str, thumbnail_product_retailer_id: Option<&str>) -> Self {
        let parameters = thumbnail_product_retailer_id.map(|id| InteractiveActionParameters {
            thumbnail_product_retailer_id: Some(id.into()),
            ..Default::default()
        });
        Self::new(
            InteractiveType::CatalogMessage,
            InteractiveAction::named("catalog_message", parameters),
            body_text,
        )
    }

    pub fn with_body(mut self, text: &str) -> Self {
        self.body = Some(InteractiveBody::new(text));
        self
    }

    pub fn with_header(mut self, header: InteractiveHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn with_footer(mut self, text: &str) -> Self {
        self.footer = Some(InteractiveFooter { text: text.into() });
        self
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct InteractiveAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    button: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buttons: Option<Vec<InteractiveActionButton>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<InteractiveActionParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_retailer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sections: Option<Vec<InteractiveActionSection>>,
}

impl InteractiveAction {
    pub fn new_buttons(buttons: Vec<InteractiveActionButton>) -> Self {
        InteractiveAction {
            buttons: Some(buttons),
            ..Default::default()
        }
    }

    pub fn new_list(button: &str, sections: Vec<InteractiveActionSection>) -> Self {
        InteractiveAction {
            button: Some(button.into()),
            sections: Some(sections),
            ..Default::default()
        }
    }

    fn named(name: &str, parameters: Option<InteractiveActionParameters>) -> Self {
        InteractiveAction {
            name: Some(name.into()),
            parameters,
            ..Default::default()
        }
    }
}

/// Parameters of named actions (`cta_url`, `flow`, `catalog_message`)
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct InteractiveActionParameters {
    // cta_url
    #[serde(skip_serializing_if = "Option::is_none")]
    display_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,

    // catalog_message
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_product_retailer_id: Option<String>,

    // flow
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_message_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_cta: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_action_payload: Option<FlowActionPayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<String>,
}

/// Flow to open from a flow message, see [`Interactive::for_flow`]
#[derive(Debug, Clone)]
pub struct FlowAction {
    flow_id: Option<String>,
    flow_name: Option<String>,
    flow_cta: String,
    flow_token: Option<String>,
    flow_action: Option<String>,
    flow_action_payload: Option<FlowActionPayload>,
    draft: bool,
}

impl FlowAction {
    /// `flow_cta` is the button text
    pub fn by_id(flow_id: &str, flow_cta: &str) -> Self {
        Self::new(Some(flow_id.into()), None, flow_cta)
    }

    /// `flow_cta` is the button text
    pub fn by_name(flow_name: &str, flow_cta: &str) -> Self {
        Self::new(None, Some(flow_name.into()), flow_cta)
    }

    fn new(flow_id: Option<String>, flow_name: Option<String>, flow_cta: &str) -> Self {
        Self {
            flow_id,
            flow_name,
            flow_cta: flow_cta.into(),
            flow_token: None,
            flow_action: None,
            flow_action_payload: None,
            draft: false,
        }
    }

    /// Token that identifies this flow session, sent back to your endpoint / webhook
    pub fn flow_token(mut self, flow_token: &str) -> Self {
        self.flow_token = Some(flow_token.into());
        self
    }

    /// Send the draft version of the flow (default is the published version)
    pub fn draft(mut self) -> Self {
        self.draft = true;
        self
    }

    /// Open the flow at `screen`, with optional initial `data` for that screen
    pub fn navigate(mut self, screen: &str, data: Option<serde_json::Value>) -> Self {
        self.flow_action = Some("navigate".into());
        self.flow_action_payload = Some(FlowActionPayload {
            screen: screen.into(),
            data,
        });
        self
    }

    /// Request the first screen from your flow endpoint
    pub fn data_exchange(mut self) -> Self {
        self.flow_action = Some("data_exchange".into());
        self.flow_action_payload = None;
        self
    }

    fn into_parameters(self) -> InteractiveActionParameters {
        InteractiveActionParameters {
            flow_message_version: Some(FLOW_MESSAGE_VERSION.into()),
            flow_token: self.flow_token,
            flow_id: self.flow_id,
            flow_name: self.flow_name,
            flow_cta: Some(self.flow_cta),
            flow_action: self.flow_action,
            flow_action_payload: self.flow_action_payload,
            mode: self.draft.then(|| "draft".into()),
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FlowActionPayload {
    screen: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
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
pub struct InteractiveFooter {
    text: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InteractiveHeader {
    #[serde(rename = "type")]
    header_type: InteractiveHeaderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Media>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video: Option<Media>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document: Option<Media>,
}

impl InteractiveHeader {
    fn new(header_type: InteractiveHeaderType) -> Self {
        Self {
            header_type,
            text: None,
            image: None,
            video: None,
            document: None,
        }
    }

    pub fn text(text: &str) -> Self {
        Self {
            text: Some(text.into()),
            ..Self::new(InteractiveHeaderType::Text)
        }
    }

    pub fn image(image: Media) -> Self {
        Self {
            image: Some(image),
            ..Self::new(InteractiveHeaderType::Image)
        }
    }

    pub fn video(video: Media) -> Self {
        Self {
            video: Some(video),
            ..Self::new(InteractiveHeaderType::Video)
        }
    }

    pub fn document(document: Media) -> Self {
        Self {
            document: Some(document),
            ..Self::new(InteractiveHeaderType::Document)
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum InteractiveHeaderType {
    Text,
    Image,
    Video,
    Document,
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
    CtaUrl,
    List,
    Product,
    ProductList,
    Flow,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InteractiveActionSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    product_items: Option<Vec<InteractiveProductItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rows: Option<Vec<InteractiveActionSectionRow>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

impl InteractiveActionSection {
    pub fn new(rows: Vec<InteractiveActionSectionRow>) -> Self {
        Self {
            product_items: None,
            rows: Some(rows),
            title: None,
        }
    }

    pub fn with_title(rows: Vec<InteractiveActionSectionRow>, title: &str) -> Self {
        Self {
            product_items: None,
            rows: Some(rows),
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

/// Section of a product list message, see [`Interactive::for_product_list`]
#[derive(Debug, Clone)]
pub struct ProductSection {
    title: String,
    product_retailer_ids: Vec<String>,
}

impl ProductSection {
    pub fn new(title: &str, product_retailer_ids: &[&str]) -> Self {
        Self {
            title: title.into(),
            product_retailer_ids: product_retailer_ids.iter().map(|&id| id.into()).collect(),
        }
    }
}

impl From<ProductSection> for InteractiveActionSection {
    fn from(section: ProductSection) -> Self {
        Self {
            product_items: Some(
                section
                    .product_retailer_ids
                    .into_iter()
                    .map(|product_retailer_id| InteractiveProductItem {
                        product_retailer_id,
                    })
                    .collect(),
            ),
            rows: None,
            title: Some(section.title),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InteractiveProductItem {
    product_retailer_id: String,
}

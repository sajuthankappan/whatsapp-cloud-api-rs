use serde::{Deserialize, Serialize};

use super::message::StatusCode;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NotificationPayload {
    pub object: String,
    pub entry: Vec<Entry>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Entry {
    pub id: String,
    pub changes: Vec<Change>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Change {
    pub value: Value,
    pub field: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Value {
    pub contacts: Option<Vec<Contact>>,
    pub errors: Option<Vec<Error>>,
    pub messaging_product: String,
    pub metadata: Metadata,
    pub messages: Option<Vec<NotificationMessage>>,
    pub statuses: Option<Vec<Status>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Contact {
    pub wa_id: String,
    pub profile: Profile,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Metadata {
    pub display_phone_number: String,
    pub phone_number_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NotificationMessage {
    pub from: String,
    pub id: String,
    pub context: Option<Context>,
    pub errors: Option<Vec<Error>>,
    pub timestamp: String,

    #[serde(rename = "type")]
    pub message_type: NotificationMessageType,
    pub audio: Option<Audio>,
    pub button: Option<Button>,
    pub document: Option<Document>,
    pub text: Option<Text>,
    pub image: Option<Image>,
    pub interactive: Option<Interactive>,
    pub order: Option<Order>,
    pub sticker: Option<Sticker>,
    pub system: Option<System>,
    pub video: Option<Video>,
    pub location: Option<Location>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Context {
    pub forwarded: Option<bool>,
    pub frequently_forwarded: Option<bool>,
    pub from: String,
    pub id: String,
    pub referred_product: Option<ReferredProduct>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReferredProduct {
    pub catalog_id: String,
    pub product_retailer_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Error {
    pub code: i32,
    pub title: String,
    // TODO: Add more fields from v16.0 and newer
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NotificationMessageType {
    Audio,
    Button,
    Document,
    Text,
    Image,
    Interactive,
    Order,
    Sticker,
    System,
    Unknown,
    Video,
    Unsupported,
    Location,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Audio {
    pub id: String,
    pub mime_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Button {
    pub payload: String,
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Document {
    pub caption: Option<String>,
    pub filename: String,
    pub sha256: String,
    pub mime_type: String,
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Image {
    pub caption: Option<String>,
    pub sha256: String,
    pub id: String,
    pub mime_type: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Interactive {
    pub button_reply: Option<ButtonReply>,
    pub list_reply: Option<ListReply>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ButtonReply {
    pub id: String,
    pub title: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ListReply {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Order {
    pub catalog_id: String,
    pub product_items: Vec<ProductItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProductItem {
    pub product_retailer_id: String,
    pub quantity: String,
    pub item_price: String,
    pub currency: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Sticker {
    pub mime_type: String,
    pub sha256: String,
    pub id: String,
    pub animated: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct System {
    pub body: String,
    pub identity: String,
    pub new_wa_id: Option<String>,
    pub wa_id: Option<String>,
    pub system_type: String,
    pub customer: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Text {
    pub body: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Video {
    pub caption: Option<String>,
    pub filename: String,
    pub sha256: String,
    pub id: String,
    pub mime_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub address: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Status {
    pub biz_opaque_callback_data: Option<String>,
    pub conversation: Option<Conversation>,
    pub errors: Option<Vec<Error>>,
    pub id: String,
    pub pricing: Option<Pricing>,
    pub recipient_id: String,
    pub status: StatusCode,
    pub timestamp: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Conversation {
    pub id: String,
    pub origin: Origin,
    // TODO: Other fields
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Origin {
    #[serde(rename = "type")]
    pub origin_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Pricing {
    pub pricing_model: String,
    // TODO: Other fields
}

use serde::{Deserialize, Serialize};

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
    pub messaging_product: String,
    pub metadata: Metadata,
    pub messages: Vec<NotificationMessage>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Metadata {
    pub display_phone_number: String,
    pub phone_number_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum NotificationMessage {
    Audio,
    Image,
    Text(TextMessage),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TextMessage {
    pub from: String,
    pub id: String,
    pub text: Text,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Text {
    pub body: String,
}

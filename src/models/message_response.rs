use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub contacts: Vec<ContactResponse>,
    pub messages: Vec<CreatedMessage>,
    pub messaging_product: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatedMessage {
    pub id: String,
    pub message_status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContactResponse {
    pub input: String,
    pub wa_id: String,
}

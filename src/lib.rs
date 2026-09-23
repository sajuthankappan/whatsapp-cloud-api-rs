mod error;
pub mod models;
mod whatsapp_client;

pub use crate::whatsapp_client::WhatsappClient;
pub use error::{ApiError, ApiErrorData, WhatsappError};

pub const WHATSAPP: &str = "whatsapp";

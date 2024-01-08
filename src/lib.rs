mod error;
pub mod models;
mod whatspapp_client;

pub use crate::whatspapp_client::WhatasppClient;
pub use error::WhatsappError;

pub const WHATSAPP: &str = "whatsapp";

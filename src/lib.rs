// WhatsappClient is deprecated for downstream users only (see its #[deprecated] note).
#![allow(deprecated)]

mod error;
pub mod models;
mod whatsapp_client;

pub use crate::whatsapp_client::WhatsappClient;
pub use error::WhatsappError;

pub const WHATSAPP: &str = "whatsapp";

use serde_json::Value;

use crate::{models::Message, WhatsappError};

const WHATSAPP_API_URL: &str = "https://graph.facebook.com/v13.0/105940028793862/messages";

pub struct WhatasppClient {
    access_token: String,
}

impl WhatasppClient {
    pub fn new(access_token: &str) -> Self {
        Self {
            access_token: access_token.into(),
        }
    }

    pub async fn send_message(&self, message: &Message) -> Result<Value, WhatsappError> {
        http_client::post(WHATSAPP_API_URL, &self.access_token, message).await
    }
}

mod http_client {
    use reqwest::StatusCode;
    use serde::{de::DeserializeOwned, Serialize};

    use crate::WhatsappError;

    pub async fn post<T, U>(url: &str, bearer_token: &str, data: &T) -> Result<U, WhatsappError>
    where
        T: Serialize + ?Sized,
        U: DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .bearer_auth(&bearer_token)
            .json(&data)
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK | StatusCode::CREATED => {
                let json = resp.json::<U>().await?;
                Ok(json)
            }
            _ => {
                log::warn!("{:?}", &resp);
                let error_text = &resp.text().await?;
                log::warn!("{:?}", &error_text);
                Err(WhatsappError::UnexpectedError(error_text.to_string()))
            }
        }
    }
}

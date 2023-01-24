use crate::{
    models::{Message, MessageResponse},
    WhatsappError,
};

pub struct WhatasppClient {
    access_token: String,
    phone_number_id: String,
}

impl WhatasppClient {
    pub fn new(access_token: &str, phone_number_id: &str) -> Self {
        Self {
            access_token: access_token.into(),
            phone_number_id: phone_number_id.into(),
        }
    }

    pub async fn send_message(&self, message: &Message) -> Result<MessageResponse, WhatsappError> {
        http_client::post(&self.whatsapp_api_url(), &self.access_token, message).await
    }

    fn whatsapp_api_url(&self) -> String {
        format!(
            "https://graph.facebook.com/v15.0/{}/messages",
            self.phone_number_id
        )
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

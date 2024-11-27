use crate::{
    models::{MediaResponse, Message, MessageResponse, MessageStatus, MessageStatusResponse},
    WhatsappError,
};

const FACEBOOK_GRAPH_API_BASE_URL: &str = "https://graph.facebook.com/v17.0";

pub struct WhatsappClient {
    access_token: String,
    phone_number_id: String,
}

impl WhatsappClient {
    pub fn new(access_token: &str, phone_number_id: &str) -> Self {
        Self {
            access_token: access_token.into(),
            phone_number_id: phone_number_id.into(),
        }
    }

    pub fn set_access_token(&mut self, access_token: &str) {
        self.access_token = access_token.into();
    }

    pub fn set_phone_number_id(&mut self, phone_number_id: &str) {
        self.access_token = phone_number_id.into();
    }

    pub async fn send_message(&self, message: &Message) -> Result<MessageResponse, WhatsappError> {
        http_client::post(&self.messages_api_url(), &self.access_token, message).await
    }

    pub async fn mark_message_as_read(
        &self,
        message_id: &str,
    ) -> Result<MessageStatusResponse, WhatsappError> {
        let message_status = MessageStatus::for_read(message_id);
        http_client::post(
            &self.messages_api_url(),
            &self.access_token,
            &message_status,
        )
        .await
    }

    pub async fn get_media(&self, media_id: &str) -> Result<MediaResponse, WhatsappError> {
        http_client::get(&self.media_api_url(media_id), &self.access_token).await
    }

    fn messages_api_url(&self) -> String {
        format!(
            "{FACEBOOK_GRAPH_API_BASE_URL}/{}/messages",
            self.phone_number_id
        )
    }

    fn media_api_url(&self, media_id: &str) -> String {
        format!("{FACEBOOK_GRAPH_API_BASE_URL}/{media_id}")
    }
}

mod http_client {
    use reqwest::StatusCode;
    use serde::{de::DeserializeOwned, Serialize};

    use crate::WhatsappError;

    pub async fn get<U>(url: &str, bearer_token: &str) -> Result<U, WhatsappError>
    where
        U: DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let resp = client.get(url).bearer_auth(bearer_token).send().await?;

        match resp.status() {
            StatusCode::OK => {
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

    pub async fn post<T, U>(url: &str, bearer_token: &str, data: &T) -> Result<U, WhatsappError>
    where
        T: Serialize + ?Sized,
        U: DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .bearer_auth(bearer_token)
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

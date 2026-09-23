use std::error::Error;

use serde::Deserialize;

#[derive(Debug)]
#[non_exhaustive]
pub enum WhatsappError {
    ReqwestError(reqwest::Error),
    /// Error response returned by the Graph API, e.g. `{"error": {"message": ..., "code": ...}}`
    ApiError(Box<ApiError>),
    /// Non-success response whose body is not a Graph API error
    UnexpectedError(String),
}

/// Graph API error, see <https://developers.facebook.com/docs/graph-api/guides/error-handling>
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ApiError {
    /// HTTP status code of the response
    #[serde(skip)]
    pub status: u16,
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub code: i64,
    pub error_subcode: Option<i64>,
    pub error_data: Option<ApiErrorData>,
    pub error_user_title: Option<String>,
    pub error_user_msg: Option<String>,
    pub fbtrace_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ApiErrorData {
    pub messaging_product: Option<String>,
    pub details: Option<String>,
}

#[derive(Deserialize)]
struct ApiErrorResponse {
    error: ApiError,
}

impl WhatsappError {
    pub(crate) fn from_response(status: u16, body: String) -> Self {
        match serde_json::from_str::<ApiErrorResponse>(&body) {
            Ok(ApiErrorResponse { mut error }) => {
                error.status = status;
                WhatsappError::ApiError(Box::new(error))
            }
            Err(_) => WhatsappError::UnexpectedError(body),
        }
    }
}

impl std::fmt::Display for WhatsappError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WhatsappError::ReqwestError(e) => e.fmt(f),
            WhatsappError::ApiError(e) => e.fmt(f),
            WhatsappError::UnexpectedError(e) => f.write_str(e),
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (code {}", self.message, self.code)?;
        if let Some(subcode) = self.error_subcode {
            write!(f, ", subcode {subcode}")?;
        }
        f.write_str(")")?;
        if let Some(details) = self.error_data.as_ref().and_then(|d| d.details.as_ref()) {
            write!(f, ": {details}")?;
        }
        Ok(())
    }
}

impl Error for WhatsappError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            WhatsappError::ReqwestError(e) => Some(e),
            WhatsappError::ApiError(_) | WhatsappError::UnexpectedError(_) => None,
        }
    }
}

impl Error for ApiError {}

impl From<reqwest::Error> for WhatsappError {
    fn from(e: reqwest::Error) -> Self {
        WhatsappError::ReqwestError(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_graph_api_error() {
        let body = r#"{"error":{"message":"(#131047) Re-engagement message","type":"OAuthException","code":131047,"error_data":{"messaging_product":"whatsapp","details":"Message failed to send because more than 24 hours have passed since the customer last replied to this number."},"fbtrace_id":"AbCdEf"}}"#;

        let WhatsappError::ApiError(e) = WhatsappError::from_response(400, body.into()) else {
            panic!("expected ApiError");
        };
        assert_eq!(e.status, 400);
        assert_eq!(e.code, 131047);
        assert_eq!(e.error_type.as_deref(), Some("OAuthException"));
        assert_eq!(e.error_subcode, None);
        assert_eq!(e.fbtrace_id.as_deref(), Some("AbCdEf"));
        assert_eq!(
            e.to_string(),
            "(#131047) Re-engagement message (code 131047): Message failed to send because more than 24 hours have passed since the customer last replied to this number."
        );
    }

    #[test]
    fn parses_graph_api_error_with_subcode() {
        let body = r#"{"error":{"message":"Error validating access token: Session has expired","type":"OAuthException","code":190,"error_subcode":463,"fbtrace_id":"XyZ"}}"#;

        let WhatsappError::ApiError(e) = WhatsappError::from_response(401, body.into()) else {
            panic!("expected ApiError");
        };
        assert_eq!(e.status, 401);
        assert_eq!(e.code, 190);
        assert_eq!(e.error_subcode, Some(463));
        assert_eq!(
            e.to_string(),
            "Error validating access token: Session has expired (code 190, subcode 463)"
        );
    }

    #[test]
    fn falls_back_to_unexpected_error_for_non_graph_body() {
        let body = "<html>502 Bad Gateway</html>";

        let WhatsappError::UnexpectedError(text) = WhatsappError::from_response(502, body.into())
        else {
            panic!("expected UnexpectedError");
        };
        assert_eq!(text, body);
    }
}

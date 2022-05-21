use std::error::Error;

#[derive(Debug)]
pub enum WhatsappError {
    ReqwestError(reqwest::Error),
    UnexpectedError(String),
}

impl std::fmt::Display for WhatsappError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WhatsappError::ReqwestError(e) => f.write_str(e.to_string().as_str()),
            WhatsappError::UnexpectedError(e) => f.write_str(e.to_string().as_str()),
        }
    }
}

impl Error for WhatsappError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            WhatsappError::ReqwestError(e) => Some(e),
            WhatsappError::UnexpectedError(_) => None,
        }
    }
}

impl From<reqwest::Error> for WhatsappError {
  fn from(e: reqwest::Error) -> Self {
    WhatsappError::ReqwestError(e)
  }
}

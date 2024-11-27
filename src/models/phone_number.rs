use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct CodeRequestParams {
    pub code_method: CodeMethod,
    pub language: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum CodeMethod {
    SMS,
    Voice,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct CodeVerifyParams {
    pub code: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PhoneNumberResponse {
    pub success: Option<bool>,
}

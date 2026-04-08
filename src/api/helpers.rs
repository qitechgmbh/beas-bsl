use serde::Deserialize;

use crate::api::Error;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RawResponse<T> {
    Data(T),
    Err(RawError),
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawError {
    #[serde(rename = "error")]
    pub value: Error,
}

#[derive(Debug, Clone, Deserialize)]
pub struct List<T> {
    pub value: Vec<T>,
}

use crate::api;
use std::fmt;

#[derive(Debug)]
pub enum TransactionError {
    UreqError(ureq::Error),
    JsonError(serde_json::Error),
    IoError(std::io::Error),
    ApiError(api::Error),
}

impl From<ureq::Error> for TransactionError {
    fn from(err: ureq::Error) -> Self {
        TransactionError::UreqError(err)
    }
}

impl From<serde_json::Error> for TransactionError {
    fn from(err: serde_json::Error) -> Self {
        TransactionError::JsonError(err)
    }
}

impl From<std::io::Error> for TransactionError {
    fn from(err: std::io::Error) -> Self {
        TransactionError::IoError(err)
    }
}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransactionError::UreqError(e) => write!(f, "HTTP Request Error: {}", e),
            TransactionError::IoError(e) => write!(f, "IO Error: {}", e),
            TransactionError::JsonError(e) => write!(f, "JSON Deserialization Error: {}", e),
            TransactionError::ApiError(error) => write!(f, "Api error: {}", error),
        }
    }
}

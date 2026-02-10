use std::fmt;

use serde_json::{self};

use crate::api;

#[derive(Debug)]
pub enum ClientError
{
    LoginError(api::Error),
    UreqError(ureq::Error),
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    LogicError(String),
    MalformedData(String),
    ApiError(api::Error)
}

impl From<ureq::Error> for ClientError 
{
    fn from(err: ureq::Error) -> Self 
    {
        ClientError::UreqError(err)
    }
}

impl From<std::io::Error> for ClientError 
{
    fn from(err: std::io::Error) -> Self 
    {
        ClientError::IoError(err)
    }
}

impl From<serde_json::Error> for ClientError 
{
    fn from(err: serde_json::Error) -> Self 
    {
        ClientError::JsonError(err)
    }
}

impl fmt::Display for ClientError 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        match self 
        {
            ClientError::UreqError(e) => write!(f, "HTTP Request Error: {}", e),
            ClientError::IoError(e) => write!(f, "IO Error: {}", e),
            ClientError::JsonError(e) => write!(f, "JSON Deserialization Error: {}", e),
            ClientError::LogicError(s) => write!(f, "Logic Error: {}", s),
            ClientError::MalformedData(e) => write!(f, "Malformed data: {}", e),
            ClientError::LoginError(error) => write!(f, "Login error: {}", error),
            ClientError::ApiError(error) => write!(f, "Api error: {}", error),
        }
    }
}
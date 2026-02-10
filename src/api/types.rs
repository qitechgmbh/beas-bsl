use serde::Deserialize;

use crate::api::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct RawError 
{ 
    #[serde(rename = "error")]
    pub value: Error 
}

#[derive(Debug, Clone, Deserialize)]
pub struct List<T> 
{ 
    pub value: Vec<T> 
}
    
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RawResponse<T> 
{
    Data(T),
    Err(RawError),
}

pub type ListResponse<T> = RawResponse<List<T>>;

#[derive(Debug)]
pub enum Response<T> 
{
    Data(T),
    Err(Error),
}
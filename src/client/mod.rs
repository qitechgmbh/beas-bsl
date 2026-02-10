use std::fmt::Debug;

use crate::api::{Login, QueryOptions, types::RawResponse};

use serde::de::DeserializeOwned;

mod error;
mod request;
mod config;

pub use config::Credentials;
pub use config::InitError;
pub use config::ClientConfig;

pub use error::ClientError;

#[derive(Debug, Clone)]
pub struct Client 
{
    server_root: String,
    session_id:  String,
}

impl Client
{
    pub fn new(config: ClientConfig) -> Result<Self, ClientError>
    {
        let session_id: String = match config.credentials 
        {
            config::Credentials::Password(password) => Self::create_session(&config.server_root, password)?,
            config::Credentials::SessionId(id) => id,
        };
        
        Ok(Self { server_root: config.server_root, session_id })
    }
    
    fn create_session(server_root: &str, password: String) -> Result<String, ClientError>
    {
        let path = format!("{}/{}", &server_root, "Login");
        
        let body = ureq::json!({ "ServicePwd": password });
        
        let response = ureq::post(&path).send_json(body)?;
        
        let data: Login = Self::deserialize_reponse(response)?;
        
        Ok(format!("beas-sessionid={}", data.beas_session_id))
    }
    
    fn get<U: Into<String>, T: DeserializeOwned + Debug>(&self, url: U, options: QueryOptions) -> Result<T, ClientError>
    {
        let path = format!("{}/{}", &self.server_root, url.into());
        
        let mut request = ureq::get(&path);

        request = request.set("Cookie", &self.session_id);

        // TODO: validate first for syntax errors!
        for query in options.to_query_list() 
        {
            request = request.query(query.param, &query.value.as_str());
        }

        let response = request.call().map_err(ClientError::UreqError)?;
        
        Self::deserialize_reponse::<T>(response)
    }
    
    fn post<U: Into<String>, T: DeserializeOwned + Debug>(&self, url: U, body: Option<serde_json::Value>) -> Result<T, ClientError>
    {
        let path = format!("{}/{}", &self.server_root, url.into());
        
        let request = ureq::get(&path).set("Cookie", &self.session_id);

        let response = match body
        {
            Some(data) => request.send_json(data),
            None    => request.call(),
        }.map_err(ClientError::UreqError)?;

        Self::deserialize_reponse::<T>(response)
    }
    
    fn deserialize_reponse<T: DeserializeOwned + Debug>(response: ureq::Response) -> Result<T, ClientError>
    {
        let body = response.into_string()?;
        
        // println!("res: {:?}", serde_json::from_str::<T>(&body));
        
        //response.into_json::<RawResponse<T>>()
        match serde_json::from_str::<RawResponse<T>>(&body)? 
        {
            RawResponse::Data(data) => Ok(data),
            RawResponse::Err(e) => 
            {
                Err(ClientError::ApiError(e.value))
            },
        }
    }
}

impl Drop for Client
{
    fn drop(&mut self) 
    {
        let _ = self.request().logout();
    }
}
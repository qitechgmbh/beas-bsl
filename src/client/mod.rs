use std::fmt::Debug;
use serde::de::DeserializeOwned;

use crate::api::{ Login, QueryOptions, helpers::RawResponse };

mod error;
mod config;
mod endpoints;

pub use error::TransactionError;

pub use config::Config as ClientConfig;
pub use config::ConfigError;
pub use config::Credentials;

use endpoints::RootEndpoint;

/// Client for interacting with the Beas BSL
#[derive(Debug, Clone)]
pub struct Client 
{
    server_root: String,
    session_id:  String,
}

impl Client
{
    pub fn new(config: ClientConfig) -> Result<Self, TransactionError>
    {
        let session_id: String = match config.credentials 
        {
            Credentials::Password(password) => Self::create_session(&config.server_root, password)?,
            Credentials::SessionId(id)      => id,
        };
        
        Ok(Self { server_root: config.server_root, session_id })
    }
    
    pub fn single_request<'a>(&'a self) -> RootEndpoint<'a>
    {
        RootEndpoint { client: self }
    }
    
    pub fn multiple_requests()
    {
        
    }
    
    /// Attempts to create a new session by logging in via the provided password
    fn create_session(server_root: &str, password: String) -> Result<String, TransactionError>
    {
        let path = format!("{}/{}", &server_root, "Login");
        
        let body = ureq::json!({ "ServicePwd": password });
        
        let response = ureq::post(&path).send_json(body)?;
        
        let data: Login = Self::deserialize_reponse(response)?;
        
        Ok(format!("beas-sessionid={}", data.beas_session_id))
    }
    
    fn get<U: Into<String>, T: DeserializeOwned + Debug>(&self, url: U, options: QueryOptions) -> Result<T, TransactionError>
    {
        let path = format!("{}/{}", &self.server_root, url.into());
        
        let mut request = ureq::get(&path).set("Cookie", &self.session_id);

        // TODO: validate first for syntax errors!
        for query in options.to_query_list() 
        {
            request = request.query(query.param, &query.value.as_str());
        }

        let response = request.call()?;
        
        Self::deserialize_reponse::<T>(response)
    }
    
    fn post<U: Into<String>, T: DeserializeOwned + Debug>(&self, url: U, body: Option<serde_json::Value>) -> Result<T, TransactionError>
    {
        let path = format!("{}/{}", &self.server_root, url.into());
        
        let request = ureq::get(&path).set("Cookie", &self.session_id);

        let response = match body
        {
            Some(data) => request.send_json(data),
            None       => request.call(),
        }?;

        Self::deserialize_reponse::<T>(response)
    }
    
    fn deserialize_reponse<T: DeserializeOwned + Debug>(response: ureq::Response) -> Result<T, TransactionError>
    {
        let body = response.into_string()?;
        
        // println!("res: {:?}", serde_json::from_str::<T>(&body));
        //response.into_json::<RawResponse<T>>()
        
        match serde_json::from_str::<RawResponse<T>>(&body)? 
        {
            RawResponse::Data(data) => Ok(data),
            RawResponse::Err(e)     => Err(TransactionError::ApiError(e.value)),
        }
    }
}

impl Drop for Client
{
    fn drop(&mut self) 
    {
        _ = self.single_request().logout();
    }
}
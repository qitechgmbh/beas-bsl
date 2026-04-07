use reqwest::Client as HttpClient;

use crate::{ClientConfig, Credentials, TransactionError, api::Login };


mod transaction;
use transaction::TransactionHandle;

#[derive(Debug, Clone)]
pub struct Client  {
    http_client: HttpClient,
    server_root: String,
    session_id:  String,
    pending_request: Option<TransactionHandle>,
}

impl Client {
    /*
    pub fn new(config: ClientConfig) -> Result<Self, TransactionError> {
        use Credentials::*;
        
        let session_id: String = match config.credentials {
            Password(password) => Self::create_session(&config.server_root, password)?,
            SessionId(id)      => id,
        };
        
        Ok(Self { server_root: config.server_root, session_id })
    } */

    pub fn send(&mut self) -> Result<(), ()> {
        if self.pending_request.is_some() {
            return Err(());
        }



        tokio::spawn(fut);

        Ok(())
    }
}

// utils
impl Client {
    /// Attempts to create a new session by logging in via the provided password
    fn create_session(server_root: &str, password: String) -> Result<String, TransactionError> {
        let path = format!("{}/{}", &server_root, "Login");
        


        let body = ureq::json!({ "ServicePwd": password });
        
        let response = ureq::post(&path).send_json(body)?;
        
        let data: Login = Self::deserialize_reponse(response)?;
        
        Ok(format!("beas-sessionid={}", data.beas_session_id))
    }
}
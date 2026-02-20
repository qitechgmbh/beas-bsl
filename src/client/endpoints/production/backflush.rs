use crate::{
    Client, 
    TransactionError, 
    api::BackflushRequest,
};

/// Represents the Backflush API endpoint, allowing submission of backflush data.
pub struct Endpoint<'a> 
{ 
    /// Reference to the underlying client used for HTTP requests.
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a>
{
    /// Submits backflush data to the server.
    ///
    /// # Parameters
    /// - `options`: Configuration and data for the backflush operation.
    ///
    /// # Returns
    /// - `Ok(())` if the request succeeds.
    /// - `Err(TransactionError)` if the request fails.
    pub fn post(&self, options: BackflushRequest) -> Result<(), TransactionError>
    {
        const URL: &str = "Backflush";
        
        let body = serde_json::to_value(&options)?;
        
        let data: String = self.client.post(URL, Some(body))?;
        
        _ = data;
        
        Ok(()) // TODO: determine actual return data
    }
}

use crate::{ api::Logout, Client, TransactionError };

mod production;
mod quality_control;

/// Represents the root API endpoint, providing access to sub-endpoints and session operations.
pub struct RootEndpoint<'a> 
{ 
    /// Reference to the underlying client used for HTTP requests.
    pub(super) client: &'a Client 
}

impl<'a> RootEndpoint<'a>
{
    /// Returns a handle to the production endpoint.
    ///
    /// # Returns
    /// A `ProductionEndpoint` tied to the same client.
    pub fn production(&self) -> production::Endpoint<'a>
    {
        production::Endpoint { client: self.client }
    }
    
    /// Returns a handle to the quality control endpoint.
    ///
    /// # Returns
    /// A `QualityControlEndpoint` tied to the same client.
    pub fn quality_control(&self) -> quality_control::Endpoint<'a>
    {
        quality_control::Endpoint { client: self.client }
    }
    
    /// Logs out the current session.
    ///
    /// # Returns
    /// - `Ok(Logout)` if the logout request succeeds.
    /// - `Err(TransactionError)` if the request fails.
    ///
    /// # Behavior
    /// Sends a POST request to the `"Logout"` endpoint and returns the resulting `Logout` data.
    pub(crate) fn logout(&self) -> Result<Logout, TransactionError>
    {
        const URL: &str = "Logout";
        
        let data: Logout = self.client.post(URL, None)?;
        
        Ok(data)
    }
}
use crate::{
    Client, 
    TransactionError, 
    api::{ 
        QueryOptions, 
        WorkorderRouting,
        helpers::List,
    }, 
};

/// Represents the Workorder Routing API endpoint, providing access to workorder routing data.
pub struct Endpoint<'a> 
{ 
    /// Reference to the underlying client used for HTTP requests.
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a>
{
    /// Retrieves a list of workorder routing entries from the server.
    ///
    /// # Parameters
    /// - `options`: Query options to filter or select specific fields.
    ///
    /// # Returns
    /// - `Ok(Vec<WorkorderRouting>)` containing the retrieved routing entries.
    /// - `Err(TransactionError)` if the request fails.
    pub fn get(&self, options: QueryOptions) -> Result<Vec<WorkorderRouting>, TransactionError>
    {
        const URL: &str = "WorkorderRouting";
        
        let options = options.select(WorkorderRouting::fields());
        
        let data: List<WorkorderRouting> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}
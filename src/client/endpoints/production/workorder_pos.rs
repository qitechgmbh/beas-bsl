use crate::{
    Client, 
    TransactionError, 
    api::{ 
        QueryOptions, 
        WorkorderPosition,
        helpers::List,
    }, 
};

/// Represents the Workorder Position API endpoint, providing access to workorder position data.
pub struct Endpoint<'a> 
{ 
    /// Reference to the underlying client used for HTTP requests.
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a>
{
    /// Retrieves a list of workorder positions from the server.
    ///
    /// # Parameters
    /// - `options`: Query options to filter, select fields, or modify the request.
    ///
    /// # Returns
    /// - `Ok(Vec<WorkorderPosition>)` containing the list of workorder positions on success.
    /// - `Err(TransactionError)` if the request fails.
    pub fn get(&self, options: QueryOptions) -> Result<Vec<WorkorderPosition>, TransactionError>
    {
        const URL: &str = "WorkorderPos";
        
        let options = options.select(WorkorderPosition::fields());
        
        let data: List<WorkorderPosition> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}
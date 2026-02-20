use crate::{
    Client, 
    TransactionError, 
    api::{ 
        QueryOptions, 
        Workorder,
        helpers::List,
    }, 
};

/// Represents the Workorder API endpoint, providing access to workorder data.
pub struct Endpoint<'a> 
{ 
    /// Reference to the underlying client used for HTTP requests.
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a>
{
    /// Retrieves a list of workorders from the server.
    ///
    /// # Parameters
    /// - `options`: Query options to filter, select fields, or modify the request.
    ///
    /// # Returns
    /// - `Ok(Vec<Workorder>)` containing the list of workorders on success.
    /// - `Err(TransactionError)` if the request fails.
    ///
    /// # Behavior
    /// 1. Automatically selects the fields defined by `Workorder::fields()`.
    /// 2. Sends a GET request to the `"Workorder"` endpoint with the provided query options.
    /// 3. Deserializes the response into a `List<Workorder>` and returns the contained values.
    pub fn get(&self, options: QueryOptions) -> Result<Vec<Workorder>, TransactionError>
    {
        const URL: &str = "Workorder";
        
        let options = options.select(Workorder::fields());
        
        let data: List<Workorder> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}

use crate::{
    Client, 
    TransactionError, 
    api::{ 
        QueryOptions, 
        WorkorderBom,
        helpers::List,
    }, 
};

/// Represents the Workorder BOM API endpoint, providing access to workorder bill-of-materials data.
pub struct Endpoint<'a> 
{ 
    /// Reference to the underlying client used for HTTP requests.
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a>
{
    /// Retrieves a list of workorder BOM entries from the server.
    ///
    /// # Parameters
    /// - `options`: Query options to filter or select specific fields.
    ///
    /// # Returns
    /// - `Ok(Vec<WorkorderBom>)` containing the retrieved BOM entries.
    /// - `Err(TransactionError)` if the request fails.
    pub fn get(&self, options: QueryOptions) -> Result<Vec<WorkorderBom>, TransactionError>
    {
        const URL: &str = "WorkorderBom";
        
        let options = options.select(WorkorderBom::fields());
        
        let data: List<WorkorderBom> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}
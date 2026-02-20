use crate::{ Client, TransactionError, api::{QCOrder, QueryOptions, helpers::List} };

/// Represents the QC Order API endpoint, providing access to quality control orders.
pub struct Endpoint<'a> 
{ 
    /// Reference to the underlying client used for HTTP requests.
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a>
{
    /// Retrieves a list of QC orders from the server.
    ///
    /// # Parameters
    /// - `options`: Query options to filter or select specific fields.
    ///
    /// # Returns
    /// - `Ok(Vec<QCOrder>)` containing the retrieved QC orders.
    /// - `Err(TransactionError)` if the request fails.
    pub fn get(&self, options: QueryOptions) -> Result<Vec<QCOrder>, TransactionError>
    {
        const URL: &str = "QCOrder";
        
        let options = options.select(QCOrder::fields());
        
        let data: List<QCOrder> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}

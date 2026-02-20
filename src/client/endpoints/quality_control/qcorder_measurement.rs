use crate::{ Client, TransactionError, api::{QCOrderMeasurement, QueryOptions, helpers::List} };

/// Represents the QC Order Measurement API endpoint, providing access to QC measurement data.
pub struct Endpoint<'a> 
{ 
    /// Reference to the underlying client used for HTTP requests.
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a>
{
    /// Retrieves a list of QC order measurements from the server.
    ///
    /// # Parameters
    /// - `options`: Query options to filter or select specific fields.
    ///
    /// # Returns
    /// - `Ok(Vec<QCOrderMeasurement>)` containing the retrieved measurements.
    /// - `Err(TransactionError)` if the request fails.
    pub fn get(&self, options: QueryOptions) -> Result<Vec<QCOrderMeasurement>, TransactionError>
    {
        const URL: &str = "QCOrderMeasurement";
        
        let options = options.select(QCOrderMeasurement::fields());
        
        let data: List<QCOrderMeasurement> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}
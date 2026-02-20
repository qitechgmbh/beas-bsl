use crate::Client;

mod qcorder;
mod qcorder_measurement;

/// Represents the Quality Control (QC) API endpoint
pub struct Endpoint<'a> 
{ 
    /// Reference to the underlying client used for HTTP requests.
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a>
{
    /// Returns a handle to the QC order endpoint.
    pub fn qcorder(&self) -> qcorder::Endpoint<'a>
    {
        qcorder::Endpoint { client: self.client }
    }
    
    /// Returns a handle to the QC order measurement endpoint.
    pub fn qcorder_measurement(&self) -> qcorder_measurement::Endpoint<'a>
    {
        qcorder_measurement::Endpoint { client: self.client }
    }
}
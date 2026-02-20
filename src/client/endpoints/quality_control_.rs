use crate::{
    Client, 
    api::{
        QCOrder, 
        QCOrderMeasurement, 
        QueryOptions,
        types::List
    }, 
    client::error::TransactionError, 
};

// root
pub struct Endpoint<'a> { pub(crate) client: &'a Client }

impl<'a> Endpoint<'a>
{
    pub fn qcorder(&self) -> QCOrderEndpoint<'a>
    {
        QCOrderEndpoint { client: self.client }
    }
    
    pub fn qcorder_measurement(&self) -> QCOrderMeasurementEndpoint<'a>
    {
        QCOrderMeasurementEndpoint { client: self.client }
    }
}

// QCOrder
pub struct QCOrderEndpoint<'a> { client: &'a Client }

impl<'a> QCOrderEndpoint<'a>
{
    pub fn get(&self, options: QueryOptions) -> Result<Vec<QCOrder>, TransactionError>
    {
        const URL: &str = "QCOrder";
        
        let options = options.select(QCOrder::fields());
        
        let data: List<QCOrder> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}

// QCOrderMeasurement
pub struct QCOrderMeasurementEndpoint<'a> { client: &'a Client }

impl<'a> QCOrderMeasurementEndpoint<'a>
{
    pub fn get(&self, options: QueryOptions) -> Result<Vec<QCOrderMeasurement>, TransactionError>
    {
        const URL: &str = "QCOrderMeasurement";
        
        let options = options.select(QCOrderMeasurement::fields());
        
        let data: List<QCOrderMeasurement> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}
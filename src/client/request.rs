use crate::{
    Client, 
    ClientError, 
    api::{
        BackflushOptions, Logout, QCOrder, QCOrderMeasurement, QueryOptions, Workorder, WorkorderBom, WorkorderPosition, WorkorderRouting, types::List
    }, 
};

pub struct RootEndpoint<'a>        { client: &'a Client }

// production endpoints
pub struct ProductionEndpoint<'a>   { client: &'a Client }
pub struct WorkorderEndpoint<'a>    { client: &'a Client }
pub struct BackflushEndpoint<'a>    { client: &'a Client }
pub struct WorkorderPosEndpoint<'a> { client: &'a Client }
pub struct WorkorderBomEndpoint<'a> { client: &'a Client }
pub struct WorkorderRoutingEndpoint<'a> { client: &'a Client }

// quality_control endpoints
pub struct QualityControlEndpoint<'a>     { client: &'a Client }
pub struct QCOrderEndpoint<'a>            { client: &'a Client }
pub struct QCOrderMeasurementEndpoint<'a> { client: &'a Client }

impl Client
{
    pub fn request<'a>(&'a self) -> RootEndpoint<'a>
    {
        RootEndpoint { client: self }
    }
}

impl<'a> RootEndpoint<'a>
{
    pub fn production(&self) -> ProductionEndpoint<'a>
    {
        ProductionEndpoint { client: self.client }
    }
    
    pub fn quality_control(&self) -> QualityControlEndpoint<'a>
    {
        QualityControlEndpoint { client: self.client }
    }
    
    pub(crate) fn logout(&self) -> Result<Logout, ClientError>
    {
        const URL: &str = "Logout";
        
        let data: Logout = self.client.post(URL, None)?;
        
        Ok(data)
    }
}

impl<'a> ProductionEndpoint<'a>
{
    pub fn workorder(&self) -> WorkorderEndpoint<'a>
    {
        WorkorderEndpoint { client: self.client }
    }
    
    pub fn workorder_pos(&self) -> WorkorderPosEndpoint<'a>
    {
        WorkorderPosEndpoint { client: self.client }
    }
    
    pub fn workorder_bom(&self) -> WorkorderBomEndpoint<'a>
    {
        WorkorderBomEndpoint { client: self.client }
    }
    
    pub fn workorder_routing(&self) -> WorkorderRoutingEndpoint<'a>
    {
        WorkorderRoutingEndpoint { client: self.client }
    }
    
    pub fn backflush(&self) -> BackflushEndpoint<'a>
    {
        BackflushEndpoint { client: self.client }
    }
}

impl<'a> WorkorderEndpoint<'a>
{
    pub fn get(&self, options: QueryOptions) -> Result<Vec<Workorder>, ClientError>
    {
        const URL: &str = "Workorder";
        
        let options = options.select(Workorder::fields());
        
        let data: List<Workorder> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}

impl<'a> WorkorderPosEndpoint<'a>
{
    pub fn get(&self, options: QueryOptions) -> Result<Vec<WorkorderPosition>, ClientError>
    {
        const URL: &str = "WorkorderPos";
        
        let options = options.select(WorkorderPosition::fields());
        
        let data: List<WorkorderPosition> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}

impl<'a> WorkorderBomEndpoint<'a>
{
    pub fn get(&self, options: QueryOptions) -> Result<Vec<WorkorderBom>, ClientError>
    {
        const URL: &str = "WorkorderBom";
        
        let options = options.select(WorkorderBom::fields());
        
        let data: List<WorkorderBom> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}

impl<'a> WorkorderRoutingEndpoint<'a>
{
    pub fn get(&self, options: QueryOptions) -> Result<Vec<WorkorderRouting>, ClientError>
    {
        const URL: &str = "WorkorderRouting";
        
        let options = options.select(WorkorderRouting::fields());
        
        let data: List<WorkorderRouting> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}

impl<'a> BackflushEndpoint<'a>
{
    pub fn post(&self, options: BackflushOptions) -> Result<Vec<QCOrder>, ClientError>
    {
        const URL: &str = "Backflush";
        
        //TODO: maybe don't unwrap...
        let body = serde_json::to_value(&options)?;
        
        let data: List<QCOrder> = self.client.post(URL, Some(body))?;
        
        Ok(data.value)
    }
}

impl<'a> QualityControlEndpoint<'a>
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

impl<'a> QCOrderEndpoint<'a>
{
    pub fn get(&self, options: QueryOptions) -> Result<Vec<QCOrder>, ClientError>
    {
        const URL: &str = "QCOrder";
        
        let options = options.select(QCOrder::fields());
        
        let data: List<QCOrder> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}

impl<'a> QCOrderMeasurementEndpoint<'a>
{
    pub fn get(&self, options: QueryOptions) -> Result<Vec<QCOrderMeasurement>, ClientError>
    {
        const URL: &str = "QCOrderMeasurement";
        
        let options = options.select(QCOrderMeasurement::fields());
        
        let data: List<QCOrderMeasurement> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}
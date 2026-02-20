use crate::{
    Client, 
    api::{
        BackflushConfig, 
        QueryOptions, 
        Workorder, 
        WorkorderBom, 
        WorkorderPosition, 
        WorkorderRouting, 
        types::List
    }, client::error::TransactionError, 
};

// root
pub struct Endpoint<'a> { pub(crate) client: &'a Client }

impl<'a> Endpoint<'a>
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

// Workorder
pub struct WorkorderEndpoint<'a> { client: &'a Client }

impl<'a> WorkorderEndpoint<'a>
{
    pub fn get(&self, options: QueryOptions) -> Result<Vec<Workorder>, TransactionError>
    {
        const URL: &str = "Workorder";
        
        let options = options.select(Workorder::fields());
        
        let data: List<Workorder> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}

// WorkorderPos
pub struct WorkorderPosEndpoint<'a> { client: &'a Client }

impl<'a> WorkorderPosEndpoint<'a>
{
    pub fn get(&self, options: QueryOptions) -> Result<Vec<WorkorderPosition>, TransactionError>
    {
        const URL: &str = "WorkorderPos";
        
        let options = options.select(WorkorderPosition::fields());
        
        let data: List<WorkorderPosition> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}

// WorkorderBom
pub struct WorkorderBomEndpoint<'a> { client: &'a Client }

impl<'a> WorkorderBomEndpoint<'a>
{
    pub fn get(&self, options: QueryOptions) -> Result<Vec<WorkorderBom>, TransactionError>
    {
        const URL: &str = "WorkorderBom";
        
        let options = options.select(WorkorderBom::fields());
        
        let data: List<WorkorderBom> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}

// WorkorderRouting
pub struct WorkorderRoutingEndpoint<'a> { client: &'a Client }

impl<'a> WorkorderRoutingEndpoint<'a>
{
    pub fn get(&self, options: QueryOptions) -> Result<Vec<WorkorderRouting>, TransactionError>
    {
        const URL: &str = "WorkorderRouting";
        
        let options = options.select(WorkorderRouting::fields());
        
        let data: List<WorkorderRouting> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}

// Backflush
pub struct BackflushEndpoint<'a> { client: &'a Client }

impl<'a> BackflushEndpoint<'a>
{
    pub fn post(&self, options: BackflushConfig) -> Result<(), TransactionError>
    {
        const URL: &str = "Backflush";
        
        let body = serde_json::to_value(&options)?;
        
        let data: String = self.client.post(URL, Some(body))?;
        
        _ = data;
        
        Ok(()) //TODO: determine return
    }
}
use crate::{
    Client, 
    TransactionError, 
    api::{ 
        QueryOptions, 
        TimeReceipt,
        helpers::List,
    }, 
};

pub struct Endpoint<'a> 
{ 
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a>
{
    pub fn get(&self, options: QueryOptions) -> Result<Vec<TimeReceipt>, TransactionError>
    {
        const URL: &str = "TimeReceipt";
        
        let options = options.select(TimeReceipt::fields());
        
        let data: List<TimeReceipt> = self.client.get(URL, options)?;
        
        Ok(data.value)
    }
}
use crate::{
    Client, TransactionError,
    api::{QueryOptions, WorkorderRouting, helpers::List},
};

pub struct Endpoint<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a> {
    pub fn get(&self, options: QueryOptions) -> Result<Vec<WorkorderRouting>, TransactionError> {
        const URL: &str = "WorkorderRouting";

        let options = options.select(WorkorderRouting::fields());

        let data: List<WorkorderRouting> = self.client.get(URL, options)?;

        Ok(data.value)
    }
}

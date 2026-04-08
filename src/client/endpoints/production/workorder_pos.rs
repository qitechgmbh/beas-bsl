use crate::{
    Client, TransactionError,
    api::{QueryOptions, WorkorderPosition, helpers::List},
};

pub struct Endpoint<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a> {
    pub fn get(&self, options: QueryOptions) -> Result<Vec<WorkorderPosition>, TransactionError> {
        const URL: &str = "WorkorderPos";

        let options = options.select(WorkorderPosition::fields());

        let data: List<WorkorderPosition> = self.client.get(URL, options)?;

        Ok(data.value)
    }
}

use crate::{
    Client, TransactionError,
    api::{QueryOptions, WorkorderBom, helpers::List},
};

pub struct Endpoint<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a> {
    pub fn get(&self, options: QueryOptions) -> Result<Vec<WorkorderBom>, TransactionError> {
        const URL: &str = "WorkorderBom";

        let options = options.select(WorkorderBom::fields());

        let data: List<WorkorderBom> = self.client.get(URL, options)?;

        Ok(data.value)
    }
}

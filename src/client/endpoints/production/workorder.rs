use crate::{
    Client, TransactionError,
    api::{QueryOptions, Workorder, helpers::List},
};

pub struct Endpoint<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a> {
    pub fn get(&self, options: QueryOptions) -> Result<Vec<Workorder>, TransactionError> {
        const URL: &str = "Workorder";

        let options = options.select(Workorder::fields());

        let data: List<Workorder> = self.client.get(URL, options)?;

        Ok(data.value)
    }
}

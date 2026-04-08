use crate::{
    Client, TransactionError,
    api::{QCOrder, QueryOptions, helpers::List},
};

pub struct Endpoint<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a> {
    pub fn get(&self, options: QueryOptions) -> Result<Vec<QCOrder>, TransactionError> {
        const URL: &str = "QCOrder";

        let options = options.select(QCOrder::fields());

        let data: List<QCOrder> = self.client.get(URL, options)?;

        Ok(data.value)
    }
}

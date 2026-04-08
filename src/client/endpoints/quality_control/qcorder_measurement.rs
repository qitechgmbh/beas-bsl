use crate::{
    Client, TransactionError,
    api::{QCOrderMeasurement, QueryOptions, helpers::List},
};

pub struct Endpoint<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a> {
    pub fn get(&self, options: QueryOptions) -> Result<Vec<QCOrderMeasurement>, TransactionError> {
        const URL: &str = "QCOrderMeasurement";

        let options = options.select(QCOrderMeasurement::fields());

        let data: List<QCOrderMeasurement> = self.client.get(URL, options)?;

        Ok(data.value)
    }
}

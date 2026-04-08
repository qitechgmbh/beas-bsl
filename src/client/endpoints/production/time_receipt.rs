use crate::api::{QueryOptions, TimeReceipt, helpers::List, time_receipt::post};
use crate::{Client, TransactionError};

pub struct Endpoint<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a> {
    const URL: &'static str = "TimeReceipt";

    pub fn get(&self, options: QueryOptions) -> Result<Vec<TimeReceipt>, TransactionError> {
        let options = options.select(TimeReceipt::fields());

        let data: List<TimeReceipt> = self.client.get(Self::URL, options)?;

        Ok(data.value)
    }

    pub fn post(&self, request: post::Request) -> Result<post::Response, TransactionError> {
        let value: serde_json::Value = serde_json::to_value(request)?;

        let array_value = serde_json::Value::Array(vec![value]);

        let response: post::Response = self.client.post(Self::URL, Some(array_value))?;

        Ok(response)
    }
}

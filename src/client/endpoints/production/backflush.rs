use crate::{Client, TransactionError, api::BackflushRequest};

pub struct Endpoint<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a> {
    pub fn post(&self, options: BackflushRequest) -> Result<(), TransactionError> {
        const URL: &str = "Backflush";

        let body = serde_json::to_value(&options)?;

        let data: String = self.client.post(URL, Some(body))?;

        _ = data;

        Ok(()) // TODO: determine actual return data
    }
}

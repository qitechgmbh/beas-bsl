use crate::{Client, TransactionError, api::Logout};

mod production;
mod quality_control;

pub struct RootEndpoint<'a> {
    pub(super) client: &'a Client,
}

impl<'a> RootEndpoint<'a> {
    pub fn production(&self) -> production::Endpoint<'a> {
        production::Endpoint {
            client: self.client,
        }
    }

    pub fn quality_control(&self) -> quality_control::Endpoint<'a> {
        quality_control::Endpoint {
            client: self.client,
        }
    }

    pub(crate) fn logout(&self) -> Result<Logout, TransactionError> {
        const URL: &str = "Logout";

        let data: Logout = self.client.post(URL, None)?;
        Ok(data)
    }
}

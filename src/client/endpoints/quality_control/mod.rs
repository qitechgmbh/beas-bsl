use crate::Client;

mod qcorder;
mod qcorder_measurement;

pub struct Endpoint<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a> {
    pub fn qcorder(&self) -> qcorder::Endpoint<'a> {
        qcorder::Endpoint {
            client: self.client,
        }
    }

    pub fn qcorder_measurement(&self) -> qcorder_measurement::Endpoint<'a> {
        qcorder_measurement::Endpoint {
            client: self.client,
        }
    }
}

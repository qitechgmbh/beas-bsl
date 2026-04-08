use crate::Client;

mod backflush;
mod time_receipt;
mod workorder;
mod workorder_bom;
mod workorder_pos;
mod workorder_routing;

/// Represents the Production API endpoint, providing access to production-related operations.
pub struct Endpoint<'a> {
    /// Reference to the underlying client used for HTTP requests.
    pub(super) client: &'a Client,
}

impl<'a> Endpoint<'a> {
    /// Returns a handle to the workorder endpoint for production operations.
    pub fn workorder(&self) -> workorder::Endpoint<'a> {
        workorder::Endpoint {
            client: self.client,
        }
    }

    /// Returns a handle to the workorder position endpoint for production operations.
    pub fn workorder_pos(&self) -> workorder_pos::Endpoint<'a> {
        workorder_pos::Endpoint {
            client: self.client,
        }
    }

    /// Returns a handle to the workorder BOM endpoint for production operations.
    pub fn workorder_bom(&self) -> workorder_bom::Endpoint<'a> {
        workorder_bom::Endpoint {
            client: self.client,
        }
    }

    /// Returns a handle to the workorder routing endpoint for production operations.
    pub fn workorder_routing(&self) -> workorder_routing::Endpoint<'a> {
        workorder_routing::Endpoint {
            client: self.client,
        }
    }

    /// Returns a handle to the backflush endpoint for production operations.
    pub fn time_receipt(&self) -> time_receipt::Endpoint<'a> {
        time_receipt::Endpoint {
            client: self.client,
        }
    }

    /// Returns a handle to the backflush endpoint for production operations.
    pub fn backflush(&self) -> backflush::Endpoint<'a> {
        backflush::Endpoint {
            client: self.client,
        }
    }
}

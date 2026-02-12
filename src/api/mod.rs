pub mod production;
pub mod quality_control;

mod query_options;

pub mod types;

mod error;
mod filter;

pub use error::Error;
pub use error::ErrorMessage;

pub use filter::Filter;
pub use query_options::Ordering;

pub mod return_code;

pub use query_options::QueryOptions;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Logout
{
    pub ret_code: u32,
    pub ret_text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Login
{
    pub ret_code: u32,
    pub ret_text: String,
    #[serde(rename = "beas-sessionid")]
    pub beas_session_id: String,
}

pub use production::workorder::Workorder;
pub use production::workorder_pos::WorkorderPosition;
pub use production::workorder_bom::WorkorderBom;
pub use production::workorder_routing::WorkorderRouting;
pub use production::backflush::BackflushOptions;

pub use quality_control::qcorder::QCOrder;
pub use quality_control::qcorder_measurement::QCOrderMeasurement;
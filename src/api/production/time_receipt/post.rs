use serde::{Deserialize, Serialize};

use crate::api::{Date, Time};

#[derive(Debug, Clone, Serialize, Default)]
pub struct Request
{
    #[serde(rename = "DocEntry")]
    pub doc_entry: i32,

    #[serde(rename = "LineNumber")]
    pub line_number: i32,

    #[serde(rename = "LineNumber2")]
    pub line_number2: i32,

    #[serde(rename = "LineNumber3", skip_serializing_if = "Option::is_none")]
    pub line_number3: Option<i32>,

    #[serde(rename = "PersonnelId")]
    pub personnel_id: String,

    #[serde(rename = "TimeType", skip_serializing_if = "Option::is_none")]
    pub time_type: Option<String>,

    #[serde(rename = "ResourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,

    #[serde(rename = "QuantityGood", skip_serializing_if = "Option::is_none")]
    pub quantity_good: Option<f64>,

    #[serde(rename = "QuantityGoodRUoM", skip_serializing_if = "Option::is_none")]
    pub quantity_good_ruom: Option<f64>,

    #[serde(rename = "QuantityScrap", skip_serializing_if = "Option::is_none")]
    pub quantity_scrap: Option<f64>,

    #[serde(rename = "QuantityScrapRUoM", skip_serializing_if = "Option::is_none")]
    pub quantity_scrap_ruom: Option<f64>,

    #[serde(rename = "ExternalCosts", skip_serializing_if = "Option::is_none")]
    pub external_costs: Option<f64>,

    #[serde(rename = "Remarks", skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,

    #[serde(rename = "CostElementId", skip_serializing_if = "Option::is_none")]
    pub cost_element_id: Option<String>,

    #[serde(rename = "CostCenter", skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,

    #[serde(rename = "DocDate", skip_serializing_if = "Option::is_none")]
    pub doc_date: Option<Date>,

    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Date>,

    #[serde(rename = "StartTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Date>,

    #[serde(rename = "EndTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    #[serde(rename = "FromTimeReceiptRunning", skip_serializing_if = "Option::is_none")]
    pub from_time_receipt_running: Option<i32>,

    #[serde(rename = "CloseEntry", skip_serializing_if = "Option::is_none")]
    pub close_entry: Option<bool>,

    #[serde(rename = "ManualBooking", skip_serializing_if = "Option::is_none")]
    pub manual_booking: Option<bool>,

    #[serde(rename = "TimeReceiptRunningId", skip_serializing_if = "Option::is_none")]
    pub time_receipt_running_id: Option<i32>,

    #[serde(rename = "TimeReceiptScrap")]
    pub time_receipt_scrap: Vec<ScrapEntry>,

    #[serde(rename = "Duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<f32>,

    #[serde(rename = "Barcode", skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,

    #[serde(rename = "CalculateDuration", skip_serializing_if = "Option::is_none")]
    pub calculate_duration: Option<bool>,

    #[serde(rename = "Issue", skip_serializing_if = "Option::is_none")]
    pub issue: Option<Issue>,

    #[serde(rename = "Receipt", skip_serializing_if = "Option::is_none")]
    pub receipt: Option<Receipt>, 

    #[serde(rename = "LogInfo", skip_serializing_if = "Option::is_none")]
    pub log_info: Option<String>,

    #[serde(rename = "LogInfoHtml", skip_serializing_if = "Option::is_none")]
    pub log_info_html: Option<String>,

    #[serde(rename = "UDF1", skip_serializing_if = "Option::is_none")]
    pub udf1: Option<String>,

    #[serde(rename = "UDF2", skip_serializing_if = "Option::is_none")]
    pub udf2: Option<String>,

    #[serde(rename = "UDF3", skip_serializing_if = "Option::is_none")]
    pub udf3: Option<String>,

    #[serde(rename = "UDF4", skip_serializing_if = "Option::is_none")]
    pub udf4: Option<String>,

    #[serde(rename = "UDF5", skip_serializing_if = "Option::is_none")]
    pub udf5: Option<String>,

    #[serde(rename = "UDF6", skip_serializing_if = "Option::is_none")]
    pub udf6: Option<String>,

    #[serde(rename = "UDF7", skip_serializing_if = "Option::is_none")]
    pub udf7: Option<String>,

    #[serde(rename = "UDF8", skip_serializing_if = "Option::is_none")]
    pub udf8: Option<String>,

    #[serde(rename = "UDF9", skip_serializing_if = "Option::is_none")]
    pub udf9: Option<String>,

    #[serde(rename = "UDF10", skip_serializing_if = "Option::is_none")]
    pub udf10: Option<String>,

    #[serde(rename = "UDF11", skip_serializing_if = "Option::is_none")]
    pub udf11: Option<String>,

    #[serde(rename = "UDF12", skip_serializing_if = "Option::is_none")]
    pub udf12: Option<String>,

    #[serde(rename = "UDF13", skip_serializing_if = "Option::is_none")]
    pub udf13: Option<String>,

    #[serde(rename = "UDF14", skip_serializing_if = "Option::is_none")]
    pub udf14: Option<String>,

    #[serde(rename = "UDF15", skip_serializing_if = "Option::is_none")]
    pub udf15: Option<String>,

    #[serde(rename = "FromTime", skip_serializing_if = "Option::is_none")]
    pub from_time: Option<Time>,

    #[serde(rename = "ToTime", skip_serializing_if = "Option::is_none")]
    pub to_time: Option<Time>,
}


#[derive(Debug, Clone, Serialize)]
pub struct ScrapEntry
{
    #[serde(rename = "ScrapId")]
    pub scrap_id: String,

    #[serde(rename = "Quantity")]
    pub quantity: f64,

    #[serde(rename = "ScrapReason")]
    pub scrap_reason: Option<String>,

    #[serde(rename = "Percent")]
    pub percent: Option<f64>,

    #[serde(rename = "UDF1")]
    pub udf1: Option<String>,

    #[serde(rename = "UDF2")]
    pub udf2: Option<String>,

    #[serde(rename = "UDF3")]
    pub udf3: Option<String>,

    #[serde(rename = "UDF4")]
    pub udf4: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Issue
{
    #[serde(rename = "Lines")]
    pub lines: Vec<IssueLine>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IssueLine
{
    #[serde(rename = "ItemCode")]
    pub item_code: String,

    #[serde(rename = "WhsCode")]
    pub whs_code: f64,

    #[serde(rename = "Quantity")]
    pub quantity: i32,

    #[serde(rename = "BaseLineNumber2")]
    pub base_line_number2: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct Receipt
{
    #[serde(rename = "Lines")]
    pub lines: Vec<ReceiptLine>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReceiptLine
{
    #[serde(rename = "ItemCode")]
    pub item_code: String,

    #[serde(rename = "WhsCode")]
    pub whs_code: f64,

    #[serde(rename = "Quantity")]
    pub quantity: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response
{
    #[serde(rename = "Value")]
    pub value: i32,

    #[serde(rename = "SystemNumber")]
    pub system_number: Vec<i32>,

    #[serde(rename = "LineNumber2")]
    pub values: Vec<ValueEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ValueEntry
{
    #[serde(rename = "TimeReceiptEntry")]
    pub time_receipt_entry: i32,

    #[serde(rename = "BoMIssueDocEntry")]
    pub bom_issue_doc_entry: i32,

    #[serde(rename = "BoMReceiptDocEntry")]
    pub bom_receipt_doc_entry: i32,

    #[serde(rename = "ReceiptDocEntry")]
    pub receipt_doc_entry: i32,

    #[serde(rename = "TimeReceiptJournalEntry")]
    pub time_receipt_journal_entry: i32,

    #[serde(rename = "QCOrderId")]
    pub qc_order_id: String,

    #[serde(rename = "QCOrderDocEnrty")]
    pub qc_order_doc_entry: i32,
}
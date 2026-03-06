#[derive(Debug, Clone, serde::Deserialize)]
pub struct TimeReceipt
{
    #[serde(rename = "SystemNumber")]
    pub system_number: i32,

    #[serde(rename = "DocEntry")]
    pub doc_entry: Option<i32>,

    #[serde(rename = "LineNumber")]
    pub line_number: Option<i32>,

    #[serde(rename = "LineNumber2")]
    pub line_number2: Option<i32>,

    #[serde(rename = "LineNumber3")]
    pub line_number3: Option<i32>,

    #[serde(rename = "Canceled")]
    pub canceled: Option<bool>,

    #[serde(rename = "CloseEntry")]
    pub close_entry: Option<bool>,

    #[serde(rename = "TimeType")]
    pub time_type: Option<String>,

    #[serde(rename = "PersonnelId")]
    pub personnel_id: Option<String>,

    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,

    #[serde(rename = "StartDateTime")]
    pub start_date_time: Option<String>,

    #[serde(rename = "EndDateTime")]
    pub end_date_time: Option<String>,

    #[serde(rename = "Duration")]
    pub duration: Option<f64>,

    #[serde(rename = "QuantityGood")]
    pub quantity_good: Option<f64>,

    #[serde(rename = "QuantityScrap")]
    pub quantity_scrap: Option<f64>,

    #[serde(rename = "QuantityGoodRUoM")]
    pub quantity_good_ruom: Option<f64>,

    #[serde(rename = "QuantityScrapRUoM")]
    pub quantity_scrap_ruom: Option<f64>,

    #[serde(rename = "RUoMFactor")]
    pub ruom_factor: Option<f64>,

    #[serde(rename = "RUoM")]
    pub ruom: Option<String>,

    #[serde(rename = "ExternalCosts")]
    pub external_costs: Option<f64>,

    #[serde(rename = "Remarks")]
    pub remarks: Option<String>,

    #[serde(rename = "ExternalWork")]
    pub external_work: Option<bool>,

    #[serde(rename = "CostElementId")]
    pub cost_element_id: Option<String>,

    #[serde(rename = "CostCenter")]
    pub cost_center: Option<String>,

    #[serde(rename = "ToolId")]
    pub tool_id: Option<String>,

    #[serde(rename = "StationId")]
    pub station_id: Option<String>,

    #[serde(rename = "UDF1")]
    pub udf1: Option<String>,

    #[serde(rename = "UDF2")]
    pub udf2: Option<String>,

    #[serde(rename = "UDF3")]
    pub udf3: Option<String>,

    #[serde(rename = "UDF4")]
    pub udf4: Option<String>,

    #[serde(rename = "UDF5")]
    pub udf5: Option<String>,

    #[serde(rename = "UDF6")]
    pub udf6: Option<String>,

    #[serde(rename = "UDF7")]
    pub udf7: Option<String>,

    #[serde(rename = "UDF8")]
    pub udf8: Option<String>,

    #[serde(rename = "UDF9")]
    pub udf9: Option<String>,

    #[serde(rename = "UDF10")]
    pub udf10: Option<String>,

    #[serde(rename = "UDF11")]
    pub udf11: Option<String>,

    #[serde(rename = "UDF12")]
    pub udf12: Option<String>,

    #[serde(rename = "UDF13")]
    pub udf13: Option<String>,

    #[serde(rename = "UDF14")]
    pub udf14: Option<String>,

    #[serde(rename = "UDF15")]
    pub udf15: Option<String>,

    #[serde(rename = "TimeReceiptRunningId")]
    pub time_receipt_running_id: Option<f64>,

    #[serde(rename = "DontDeleteTimeReceiptRunning")]
    pub dont_delete_time_receipt_running: Option<f64>,

    #[serde(rename = "Barcode")]
    pub barcode: Option<f64>,
}

impl TimeReceipt
{
    const FIELDS: [&str; 44] =
    [
        "SystemNumber",
        "DocEntry",
        "LineNumber",
        "LineNumber2",
        "LineNumber3",
        "Canceled",
        "CloseEntry",
        "TimeType",
        "PersonnelId",
        "ResourceId",
        "StartDateTime",
        "EndDateTime",
        "Duration",
        "QuantityGood",
        "QuantityScrap",
        "QuantityGoodRUoM",
        "QuantityScrapRUoM",
        "RUoMFactor",
        "RUoM",
        "ExternalCosts",
        "Remarks",
        "ExternalWork",
        "CostElementId",
        "CostCenter",
        "ToolId",
        "StationId",
        "UDF1",
        "UDF2",
        "UDF3",
        "UDF4",
        "UDF5",
        "UDF6",
        "UDF7",
        "UDF8",
        "UDF9",
        "UDF10",
        "UDF11",
        "UDF12",
        "UDF13",
        "UDF14",
        "UDF15",
        "TimeReceiptRunningId",
        "DontDeleteTimeReceiptRunning",
        "Barcode",
    ];

    pub fn fields() -> &'static [&'static str]
    {
        &Self::FIELDS
    }
}
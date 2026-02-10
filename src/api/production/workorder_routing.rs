#[derive(Debug, Clone, serde::Deserialize)]
pub struct WorkorderRouting
{
    #[serde(rename = "DocEntry")]
    pub doc_entry: i32,

    #[serde(rename = "LineNumber")]
    pub line_number: Option<i32>,

    #[serde(rename = "LineNumber2")]
    pub line_number2: Option<i32>,

    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,

    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,

    #[serde(rename = "OperationTypeId")]
    pub operation_type_id: Option<String>,

    #[serde(rename = "SortId")]
    pub sort_id: Option<i32>,

    #[serde(rename = "Position")]
    pub position: Option<String>,

    #[serde(rename = "ClockMandatory")]
    pub clock_mandatory: Option<bool>,

    #[serde(rename = "Barcode")]
    pub barcode: Option<String>,

    #[serde(rename = "Closed")]
    pub closed: Option<bool>,

    #[serde(rename = "RoutingColor")]
    pub routing_color: Option<i32>,

    #[serde(rename = "RoutingBitmap")]
    pub routing_bitmap: Option<String>,

    #[serde(rename = "Description")]
    pub description: Option<String>,

    #[serde(rename = "FirstLine")]
    pub first_line: Option<bool>,

    #[serde(rename = "LastLine")]
    pub last_line: Option<bool>,

    #[serde(rename = "CurrentRunningCurrentPersonnel")]
    pub current_running_current_personnel: Option<bool>,

    #[serde(rename = "CurrentRunning")]
    pub current_running: Option<bool>,

    #[serde(rename = "InterruptionId")]
    pub interruption_id: Option<String>,

    #[serde(rename = "RUoMFactor")]
    pub ruo_m_factor: Option<f64>,

    #[serde(rename = "RUoM")]
    pub ruo_m: Option<String>,

    #[serde(rename = "Instructions")]
    pub instructions: Option<String>,

    #[serde(rename = "Picture1")]
    pub picture1: Option<String>,

    #[serde(rename = "Picture2")]
    pub picture2: Option<String>,

    #[serde(rename = "Picture3")]
    pub picture3: Option<String>,

    #[serde(rename = "TotalStartTime")]
    pub total_start_time: Option<String>,

    #[serde(rename = "TransferStartTime")]
    pub transfer_start_time: Option<String>,

    #[serde(rename = "TransferEndTime")]
    pub transfer_end_time: Option<String>,

    #[serde(rename = "ProcessingStartTime")]
    pub processing_start_time: Option<String>,

    #[serde(rename = "ProcessingEndTime")]
    pub processing_end_time: Option<String>,

    #[serde(rename = "IdleStartTime")]
    pub idle_start_time: Option<String>,

    #[serde(rename = "IdleEndTime")]
    pub idle_end_time: Option<String>,

    #[serde(rename = "TotalEndTime")]
    pub total_end_time: Option<String>,

    #[serde(rename = "QuantityProduced")]
    pub quantity_produced: Option<f64>,

    #[serde(rename = "ForPersonnelId")]
    pub for_personnel_id: Option<String>,
}

impl WorkorderRouting
{
    const FIELDS: [&str; 35] =
    [
        "DocEntry",
        "LineNumber",
        "LineNumber2",
        "ResourceId",
        "CatalogId",
        "OperationTypeId",
        "SortId",
        "Position",
        "ClockMandatory",
        "Barcode",
        "Closed",
        "RoutingColor",
        "RoutingBitmap",
        "Description",
        "FirstLine",
        "LastLine",
        "CurrentRunningCurrentPersonnel",
        "CurrentRunning",
        "InterruptionId",
        "RUoMFactor",
        "RUoM",
        "Instructions",
        "Picture1",
        "Picture2",
        "Picture3",
        "TotalStartTime",
        "TransferStartTime",
        "TransferEndTime",
        "ProcessingStartTime",
        "ProcessingEndTime",
        "IdleStartTime",
        "IdleEndTime",
        "TotalEndTime",
        "QuantityProduced",
        "ForPersonnelId",
    ];

    pub fn fields() -> &'static [&'static str]
    {
        &Self::FIELDS
    }
}
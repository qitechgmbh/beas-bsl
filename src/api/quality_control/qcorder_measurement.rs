#[derive(Debug, Clone, serde::Deserialize)]
pub struct QCOrderMeasurement
{
    #[serde(rename = "DocEntry")]
    pub doc_entry: Option<i32>,

    #[serde(rename = "DocOrder")]
    pub doc_order: Option<String>,

    #[serde(rename = "DocOrder2")]
    pub doc_order2: Option<i32>,

    #[serde(rename = "LineNumber")]
    pub line_number: Option<String>,

    #[serde(rename = "LineNumber2")]
    pub line_number2: Option<i32>,

    #[serde(rename = "PosText")]
    pub pos_text: Option<String>,

    #[serde(rename = "Sort")]
    pub sort: Option<i32>,

    #[serde(rename = "Bitmap")]
    pub bitmap: Option<String>,

    #[serde(rename = "Color")]
    pub color: Option<i32>,

    #[serde(rename = "QCPickList")]
    pub qc_pick_list: Option<String>,

    #[serde(rename = "WoDocEntry")]
    pub wo_doc_entry: Option<i32>,

    #[serde(rename = "WoLineNumber")]
    pub wo_line_number: Option<i32>,

    #[serde(rename = "WoLineNumber2")]
    pub wo_line_number2: Option<i32>,

    #[serde(rename = "QCGroupId")]
    pub qc_group_id: Option<String>,

    #[serde(rename = "QCDescription")]
    pub qc_description: Option<String>,

    #[serde(rename = "QCInfo")]
    pub qc_info: Option<String>,

    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,

    #[serde(rename = "AttributeSelection")]
    pub attribute_selection: Option<String>,

    #[serde(rename = "Type")]
    pub type_: Option<String>,

    #[serde(rename = "PrintRelevant")]
    pub print_relevant: Option<bool>,

    /// Docs says it's an int... but samples
    /// yield floats.. sometimes?
    #[serde(rename = "Minimal")]
    pub minimal: Option<f64>,

    /// Docs says it's an int... but samples
    /// yield floats.. sometimes?
    #[serde(rename = "Maximum")]
    pub maximum: Option<f64>,

    /// Docs says it's an int... but samples
    /// yield floats.. sometimes?
    #[serde(rename = "DesiredValue")]
    pub desired_value: Option<f64>,

    #[serde(rename = "UoMCode")]
    pub uom_code: Option<String>,

    #[serde(rename = "Relevant")]
    pub relevant: Option<bool>,

    #[serde(rename = "MeasurementOK")]
    pub measurement_ok: Option<bool>,

    #[serde(rename = "MeasurementOKManual")]
    pub measurement_ok_manual: Option<bool>,

    /// Docs says it's an int32... but samples
    /// yield floats.. sometimes?
    #[serde(rename = "MeasurementNumber")]
    pub measurement_number: Option<f64>,

    #[serde(rename = "MeasurementString")]
    pub measurement_string: Option<String>,

    #[serde(rename = "MeasurementPersonellId")]
    pub measurement_personell_id: Option<String>,

    #[serde(rename = "MeasurementPersonellName")]
    pub measurement_personell_name: Option<String>,

    #[serde(rename = "MeasurementStationId")]
    pub measurement_station_id: Option<String>,

    #[serde(rename = "MeasurementDate")]
    pub measurement_date: Option<String>,

    #[serde(rename = "ValuationId")]
    pub valuation_id: Option<String>,

    #[serde(rename = "ValuationText")]
    pub valuation_text: Option<String>,

    #[serde(rename = "BlockageReasonId")]
    pub blockage_reason_id: Option<String>,

    #[serde(rename = "BlockageReasonText")]
    pub blockage_reason_text: Option<String>,

    #[serde(rename = "ReleaseAutomatic")]
    pub release_automatic: Option<bool>,

    #[serde(rename = "Release2")]
    pub release2: Option<bool>,

    #[serde(rename = "ReleasePersonnelId")]
    pub release_personnel_id: Option<String>,

    #[serde(rename = "ReleasePersName")]
    pub release_pers_name: Option<String>,

    #[serde(rename = "ReleaseStationId")]
    pub release_station_id: Option<String>,

    #[serde(rename = "ReleaseDate")]
    pub release_date: Option<String>,

    #[serde(rename = "Picture1")]
    pub picture1: Option<String>,

    #[serde(rename = "Picture2")]
    pub picture2: Option<String>,

    #[serde(rename = "Picture3")]
    pub picture3: Option<String>,

    #[serde(rename = "LastChangeDate")]
    pub last_change_date: Option<String>,

    #[serde(rename = "LastChangeUserId")]
    pub last_change_user_id: Option<String>,
}

impl QCOrderMeasurement
{
    const FIELDS: [&str; 48] =
    [
        "DocEntry",
        "DocOrder",
        "DocOrder2",
        "LineNumber",
        "LineNumber2",
        "PosText",
        "Sort",
        "Bitmap",
        "Color",
        "QCPickList",
        "WoDocEntry",
        "WoLineNumber",
        "WoLineNumber2",
        "QCGroupId",
        "QCDescription",
        "QCInfo",
        "ResourceId",
        "AttributeSelection",
        "Type",
        "PrintRelevant",
        "Minimal",
        "Maximum",
        "DesiredValue",
        "UoMCode",
        "Relevant",
        "MeasurementOK",
        "MeasurementOKManual",
        "MeasurementNumber",
        "MeasurementString",
        "MeasurementPersonellId",
        "MeasurementPersonellName",
        "MeasurementStationId",
        "MeasurementDate",
        "ValuationId",
        "ValuationText",
        "BlockageReasonId",
        "BlockageReasonText",
        "ReleaseAutomatic",
        "Release2",
        "ReleasePersonnelId",
        "ReleasePersName",
        "ReleaseStationId",
        "ReleaseDate",
        "Picture1",
        "Picture2",
        "Picture3",
        "LastChangeDate",
        "LastChangeUserId",
    ];

    pub fn fields() -> &'static [&'static str]
    {
        &Self::FIELDS
    }
}
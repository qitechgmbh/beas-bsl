#[derive(Debug, Clone, serde::Deserialize)]
pub struct QCOrder
{
    #[serde(rename = "DocEntry")]
    pub doc_entry: i32,

    #[serde(rename = "DocOrder")]
    pub doc_order: Option<String>,

    #[serde(rename = "DocOrder2")]
    pub doc_order2: Option<i32>,

    #[serde(rename = "DocDate")]
    pub doc_date: Option<String>,

    #[serde(rename = "ItemCode")]
    pub item_code: Option<String>,

    #[serde(rename = "ItemName")]
    pub item_name: Option<String>,

    #[serde(rename = "IVersionId")]
    pub i_version_id: Option<String>,

    #[serde(rename = "DrawingNumber")]
    pub drawing_number: Option<String>,

    #[serde(rename = "WhsCode")]
    pub whs_code: Option<String>,

    #[serde(rename = "Quantity")]
    pub quantity: Option<f64>,

    #[serde(rename = "UoMStock")]
    pub uo_m_stock: Option<String>,

    #[serde(rename = "ItemInfo")]
    pub item_info: Option<String>,

    #[serde(rename = "DistNumber")]
    pub dist_number: Option<String>,

    #[serde(rename = "QCInspectionPlanId")]
    pub qc_inspection_plan_id: Option<String>,

    #[serde(rename = "ParentQCOrder")]
    pub parent_qc_order: Option<String>,

    #[serde(rename = "ItemQcLinkId")]
    pub item_qc_link_id: Option<i32>,

    #[serde(rename = "RuleId")]
    pub rule_id: Option<String>,

    #[serde(rename = "Type")]
    pub type_: Option<String>,

    #[serde(rename = "BaseType")]
    pub base_type: Option<String>,

    #[serde(rename = "BaseDocEntry")]
    pub base_doc_entry: Option<i32>,

    #[serde(rename = "BaseDocOrder")]
    pub base_doc_order: Option<String>,

    #[serde(rename = "BaseLineNumber")]
    pub base_line_number: Option<i32>,

    #[serde(rename = "BaseLineNumber2")]
    pub base_line_number2: Option<i32>,

    #[serde(rename = "CardCode")]
    pub card_code: Option<String>,

    #[serde(rename = "PrjCode")]
    pub prj_code: Option<String>,

    #[serde(rename = "PrjUID")]
    pub prj_uid: Option<i32>,

    #[serde(rename = "CreatedUserSign")]
    pub created_user_sign: Option<i32>,

    #[serde(rename = "CreatedPersonellId")]
    pub created_personell_id: Option<String>,

    #[serde(rename = "CreatedBeasVersion")]
    pub created_beas_version: Option<String>,

    #[serde(rename = "CreatedStation")]
    pub created_station: Option<String>,

    #[serde(rename = "WoDocEntry")]
    pub wo_doc_entry: Option<i32>,

    #[serde(rename = "WoLineNumber")]
    pub wo_line_number: Option<i32>,

    #[serde(rename = "WoLineNumber2")]
    pub wo_line_number2: Option<i32>,

    #[serde(rename = "SamplesOpen")]
    pub samples_open: Option<i32>,

    #[serde(rename = "SamplesOK")]
    pub samples_ok: Option<i32>,

    #[serde(rename = "SamplesError")]
    pub samples_error: Option<i32>,

    #[serde(rename = "PrintStatus")]
    pub print_status: Option<bool>,

    #[serde(rename = "MaterialTransfer")]
    pub material_transfer: Option<i32>,

    #[serde(rename = "Release")]
    pub release: Option<bool>,

    #[serde(rename = "ReleasePersonnelId")]
    pub release_personnel_id: Option<String>,

    #[serde(rename = "ReleasePersName")]
    pub release_pers_name: Option<String>,

    #[serde(rename = "ReleaseStationId")]
    pub release_station_id: Option<String>,

    #[serde(rename = "ReleaseDate")]
    pub release_date: Option<String>,

    #[serde(rename = "ValuationId")]
    pub valuation_id: Option<String>,

    #[serde(rename = "ValuationText")]
    pub valuation_text: Option<String>,

    #[serde(rename = "BlockageReasonId")]
    pub blockage_reason_id: Option<String>,

    #[serde(rename = "BlockageReasonText")]
    pub blockage_reason_text: Option<String>,

    #[serde(rename = "Closed")]
    pub closed: Option<bool>,

    #[serde(rename = "CloseDate")]
    pub close_date: Option<String>,

    #[serde(rename = "ClosePersonellName")]
    pub close_personell_name: Option<String>,

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

impl QCOrder
{
    const FIELDS: [&str; 55] =
    [
        "DocEntry",
        "DocOrder",
        "DocOrder2",
        "DocDate",
        "ItemCode",
        "ItemName",
        "IVersionId",
        "DrawingNumber",
        "WhsCode",
        "Quantity",
        "UoMStock",
        "ItemInfo",
        "DistNumber",
        "QCInspectionPlanId",
        "ParentQCOrder",
        "ItemQcLinkId",
        "RuleId",
        "Type",
        "BaseType",
        "BaseDocEntry",
        "BaseDocOrder",
        "BaseLineNumber",
        "BaseLineNumber2",
        "CardCode",
        "PrjCode",
        "PrjUID",
        "CreatedUserSign",
        "CreatedPersonellId",
        "CreatedBeasVersion",
        "CreatedStation",
        "WoDocEntry",
        "WoLineNumber",
        "WoLineNumber2",
        "SamplesOpen",
        "SamplesOK",
        "SamplesError",
        "PrintStatus",
        "MaterialTransfer",
        "Release",
        "ReleasePersonnelId",
        "ReleasePersName",
        "ReleaseStationId",
        "ReleaseDate",
        "ValuationId",
        "ValuationText",
        "BlockageReasonId",
        "BlockageReasonText",
        "Closed",
        "CloseDate",
        "ClosePersonellName",
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
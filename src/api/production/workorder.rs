#[derive(Debug, Clone, serde::Deserialize)]
pub struct Workorder
{
    #[serde(rename = "DocEntry")]
    pub doc_entry: i32,

    #[serde(rename = "DocNum")]
    pub doc_num: Option<String>,

    #[serde(rename = "DocDate")]
    pub doc_date: Option<String>,

    #[serde(rename = "DeliveryDate")]
    pub delivery_date: Option<String>,

    #[serde(rename = "CardCode")]
    pub card_code: Option<String>,

    #[serde(rename = "CardName")]
    pub card_name: Option<String>,

    #[serde(rename = "MailAddres")]
    pub mail_address: Option<String>,

    #[serde(rename = "MailZipCod")]
    pub mail_zip_code: Option<String>,

    #[serde(rename = "MailCitiy")]
    pub mail_city: Option<String>,

    #[serde(rename = "MailCountry")]
    pub mail_country: Option<String>,

    #[serde(rename = "Phone1")]
    pub phone1: Option<String>,

    #[serde(rename = "AditionalText")]
    pub additional_text: Option<String>,

    #[serde(rename = "Lock")]
    pub lock: Option<i32>,

    #[serde(rename = "Closed")]
    pub closed: Option<i32>,

    #[serde(rename = "PriorityCode")]
    pub priority_code: Option<String>,

    #[serde(rename = "StartDate")]
    pub start_date: Option<String>,

    #[serde(rename = "EndDate")]
    pub end_date: Option<String>,

    #[serde(rename = "PrintStatus")]
    pub print_status: Option<i32>,

    #[serde(rename = "ItemCode")]
    pub item_code: Option<String>,

    #[serde(rename = "OrderBitmap")]
    pub order_bitmap: Option<String>,

    // NOTE: DATATYPE MISMATCH: found float instead of int???
    // #[serde(rename = "PlannedTime")]
    // pub planned_time: Option<i32>,

    // NOTE: DATATYPE MISMATCH: found float instead of int???
    // #[serde(rename = "WorkTime")]
    // pub work_time: Option<i32>,

    #[serde(rename = "ReservedTime")]
    pub reserved_time: Option<i32>,

    #[serde(rename = "ProductionTypeId")]
    pub production_type_id: Option<String>,

    #[serde(rename = "PrjCode")]
    pub prj_code: Option<String>,

    #[serde(rename = "PrjUID")]
    pub prj_uid: Option<i32>,

    #[serde(rename = "WorkorderStatus")]
    pub workorder_status: Option<String>,

    #[serde(rename = "BranchId")]
    pub branch_id: Option<i32>,

    #[serde(rename = "Locked")]
    pub locked: Option<bool>,

    #[serde(rename = "ApsStatus")]
    pub aps_status: Option<bool>,
}

impl Workorder
{
    const FIELDS: [&str; 30] =
    [
        "DocEntry",
        "DocNum",
        "DocDate",
        "DeliveryDate",
        "CardCode",
        "CardName",
        "MailAddres",
        "MailZipCod",
        "MailCitiy",
        "MailCountry",
        "Phone1",
        "AditionalText",
        "Lock",
        "Closed",
        "PriorityCode",
        "StartDate",
        "EndDate",
        "PrintStatus",
        "ItemCode",
        "OrderBitmap",
        "PlannedTime",
        "WorkTime",
        "ReservedTime",
        "ProductionTypeId",
        "PrjCode",
        "PrjUID",
        "WorkorderStatus",
        "BranchId",
        "Locked",
        "ApsStatus",
    ];
    
    pub fn fields() -> &'static [&'static str]
    {
        &Self::FIELDS
    }
}
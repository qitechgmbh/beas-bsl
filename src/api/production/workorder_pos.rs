#[derive(Debug, Clone, serde::Deserialize)]
pub struct WorkorderPosition
{
    #[serde(rename = "DocEntry")]
    pub doc_entry: i32,

    #[serde(rename = "LineNumber")]
    pub line_number: i32,

    #[serde(rename = "Barcode")]
    pub barcode: Option<String>,

    #[serde(rename = "Level")]
    pub level: Option<i32>,

    #[serde(rename = "ItemCode")]
    pub item_code: Option<String>,

    #[serde(rename = "ItemVersion")]
    pub item_version: Option<String>,

    #[serde(rename = "ItemName")]
    pub item_name: Option<String>,

    #[serde(rename = "Quantity")]
    pub quantity: Option<i32>,

    #[serde(rename = "QuantityConsumptionUoM")]
    pub quantity_consumption_uom: Option<i32>,

    #[serde(rename = "UoMConsumptionId")]
    pub uom_consumption_id: Option<String>,

    #[serde(rename = "UoMConsumptionFactor")]
    pub uom_consumption_factor: Option<i32>,

    #[serde(rename = "QuantityWhsUnit")]
    pub quantity_whs_unit: Option<f64>,

    #[serde(rename = "QuantityScrap")]
    pub quantity_scrap: Option<f64>,

    #[serde(rename = "QuantityProduced")]
    pub quantity_produced: Option<f64>,

    #[serde(rename = "UoMWareHouseId")]
    pub uom_warehouse_id: Option<String>,

    #[serde(rename = "WhsCode")]
    pub whs_code: Option<String>,

    #[serde(rename = "BinCode")]
    pub bin_code: Option<String>,

    #[serde(rename = "ProductionLine")]
    pub production_line: Option<String>,

    #[serde(rename = "BaseType")]
    pub base_type: Option<String>,

    #[serde(rename = "BaseDocEntry")]
    pub base_doc_entry: Option<i32>,

    #[serde(rename = "BaseDocNum")]
    pub base_doc_num: Option<i32>,

    #[serde(rename = "BaseLineNumber")]
    pub base_line_number: Option<i32>,

    #[serde(rename = "BaseLineNumber2")]
    pub base_line_number2: Option<i32>,

    #[serde(rename = "BaseWhsCode")]
    pub base_whs_code: Option<String>,

    #[serde(rename = "DrawingNumber")]
    pub drawing_number: Option<String>,

    #[serde(rename = "DIN")]
    pub din: Option<String>,

    #[serde(rename = "RawMaterialId")]
    pub raw_material_id: Option<String>,

    #[serde(rename = "MatchCode")]
    pub match_code: Option<String>,

    #[serde(rename = "Configuration")]
    pub configuration: Option<i32>,

    #[serde(rename = "ShortVariant")]
    pub short_variant: Option<String>,

    #[serde(rename = "DeliveryDate")]
    pub delivery_date: Option<String>,

    #[serde(rename = "AssignedToLineNumber")]
    pub assigned_to_line_number: Option<i32>,

    #[serde(rename = "SchemaId")]
    pub schema_id: Option<String>,

    #[serde(rename = "Closed")]
    pub closed: Option<bool>,

    #[serde(rename = "Confirmed")]
    pub confirmed: Option<bool>,

    #[serde(rename = "ChangeAllow")]
    pub change_allow: Option<bool>,

    #[serde(rename = "MaterialPlanning")]
    pub material_planning: Option<bool>,

    #[serde(rename = "MaterialBooking")]
    pub material_booking: Option<bool>,

    #[serde(rename = "CapacityPlanning")]
    pub capacity_planning: Option<bool>,

    #[serde(rename = "LoginToWorkOrder")]
    pub login_to_work_order: Option<bool>,
}

impl WorkorderPosition
{
    const FIELDS: [&str; 40] =
    [
        "DocEntry",
        "LineNumber",
        "Barcode",
        "Level",
        "ItemCode",
        "ItemVersion",
        "ItemName",
        "Quantity",
        "QuantityConsumptionUoM",
        "UoMConsumptionId",
        "UoMConsumptionFactor",
        "QuantityWhsUnit",
        "QuantityScrap",
        "QuantityProduced",
        "UoMWareHouseId",
        "WhsCode",
        "BinCode",
        "ProductionLine",
        "BaseType",
        "BaseDocEntry",
        "BaseDocNum",
        "BaseLineNumber",
        "BaseLineNumber2",
        "BaseWhsCode",
        "DrawingNumber",
        "DIN",
        "RawMaterialId",
        "MatchCode",
        "Configuration",
        "ShortVariant",
        "DeliveryDate",
        "AssignedToLineNumber",
        "SchemaId",
        "Closed",
        "Confirmed",
        "ChangeAllow",
        "MaterialPlanning",
        "MaterialBooking",
        "CapacityPlanning",
        "LoginToWorkOrder",
    ];

    pub fn fields() -> &'static [&'static str]
    {
        &Self::FIELDS
    }
}
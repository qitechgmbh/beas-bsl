#[derive(Debug, Clone, serde::Deserialize)]
pub struct WorkorderBom
{
    #[serde(rename = "DocEntry")]
    pub doc_entry: i32,

    #[serde(rename = "LineNumber")]
    pub line_number: Option<i32>,

    #[serde(rename = "LineNumber2")]
    pub line_number2: Option<i32>,

    #[serde(rename = "SortId")]
    pub sort_id: Option<i32>,

    #[serde(rename = "Position")]
    pub position: Option<String>,

    /// According to reference docs this is a Edm.i32
    /// However from multiple tests It appears to be
    /// a string... don't know why... but it is what
    /// it is... I guess
    #[serde(rename = "Barcode")]
    pub barcode: Option<String>,

    #[serde(rename = "ItemCode")]
    pub item_code: Option<String>,

    #[serde(rename = "ItemName")]
    pub item_name: Option<String>,

    #[serde(rename = "IVersionId")]
    pub i_version_id: Option<String>,

    #[serde(rename = "QuantityWhsUoM")]
    pub quantity_whs_uom: Option<f64>,

    #[serde(rename = "UoMWarehouseId")]
    pub uom_warehouse_id: Option<String>,

    #[serde(rename = "QuantityBomUoM")]
    pub quantity_bom_uom: Option<f64>,

    #[serde(rename = "UoMConsumptionId")]
    pub uom_consumption_id: Option<String>,

    #[serde(rename = "InputQuantity")]
    pub input_quantity: Option<f64>,

    #[serde(rename = "InputUnit")]
    pub input_unit: Option<String>,

    #[serde(rename = "InputFactor")]
    pub input_factor: Option<f64>,

    #[serde(rename = "QuantityFixWhsUnit")]
    pub quantity_fix_whs_unit: Option<f64>,

    #[serde(rename = "QuantityTotalWhsUnit")]
    pub quantity_total_whs_unit: Option<f64>,

    #[serde(rename = "QuantityBooked")]
    pub quantity_booked: Option<f64>,

    #[serde(rename = "QuantityReservation")]
    pub quantity_reservation: Option<f64>,

    #[serde(rename = "WhsCode")]
    pub whs_code: Option<String>,

    #[serde(rename = "BinCode")]
    pub bin_code: Option<String>,

    #[serde(rename = "RoutingPosition")]
    pub routing_position: Option<f64>,

    #[serde(rename = "DrawingNumber")]
    pub drawing_number: Option<String>,

    #[serde(rename = "DIN")]
    pub din: Option<String>,

    #[serde(rename = "RawMaterialId")]
    pub raw_material_id: Option<String>,

    #[serde(rename = "MatchCode")]
    pub match_code: Option<String>,

    #[serde(rename = "BomColor")]
    pub bom_color: Option<i32>,

    #[serde(rename = "BomBitmap")]
    pub bom_bitmap: Option<String>,

    #[serde(rename = "Closed")]
    pub closed: Option<bool>,
}

impl WorkorderBom
{
    const FIELDS: [&str; 30] =
    [
        "DocEntry",
        "LineNumber",
        "LineNumber2",
        "SortId",
        "Position",
        "Barcode",
        "ItemCode",
        "ItemName",
        "IVersionId",
        "QuantityWhsUoM",
        "UoMWarehouseId",
        "QuantityBomUoM",
        "UoMConsumptionId",
        "InputQuantity",
        "InputUnit",
        "InputFactor",
        "QuantityFixWhsUnit",
        "QuantityTotalWhsUnit",
        "QuantityBooked",
        "QuantityReservation",
        "WhsCode",
        "BinCode",
        "RoutingPosition",
        "DrawingNumber",
        "DIN",
        "RawMaterialId",
        "MatchCode",
        "BomColor",
        "BomBitmap",
        "Closed",
    ];

    pub fn fields() -> &'static [&'static str]
    {
        &Self::FIELDS
    }
}
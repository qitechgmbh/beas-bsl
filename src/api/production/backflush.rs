// https://help.beascloud.com/script202204/index.html?backflush.htm

#[derive(Debug, Clone, serde::Serialize)]
pub struct BackflushRequest
{
    /// Work oder number
    #[serde(rename = "DocEntry")]
    pub doc_entry: i32,
    
    /// Line Number
    #[serde(rename = "LineNumber")]
    pub line_number: i32,
    
    /// Optional Document date, if not today.
    #[serde(rename = "DocDate")]
    pub doc_date: Option<String>,
    
    /// set to true, if you want to close the work order
    #[serde(rename = "CloseEntry")]
    pub close_entry: bool,
    
    /// Good Quantity in Warehouse Unit
    #[serde(rename = "QuantityGood")]
    pub quantity_good: f32,
    
    /// Issue information
    #[serde(rename = "IssueLines")]
    pub issue_lines: Vec<IssueLine>,
    
    /// receipt information
    #[serde(rename = "ReceiptLines")]
    pub receipt_lines: Vec<ReceiptLine>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct IssueLine
{
    #[serde(rename = "LineNumber2")]
    pub line_number2: i32,

    #[serde(rename = "Quantity")]
    pub quantity: f32,

    #[serde(rename = "Quantity")]
    pub item_code: String,

    #[serde(rename = "Quantity")]
    pub whs_code: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ReceiptLine
{
    #[serde(rename = "Quantity")]
    pub quantity: f32,

    #[serde(rename = "Quantity")]
    pub item_code: String,

    #[serde(rename = "Quantity")]
    pub whs_code: String,
}
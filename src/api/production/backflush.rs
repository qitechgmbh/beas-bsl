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
    pub doc_date: Option<f32>,
    
    /// set to true, if you want to close the work order
    #[serde(rename = "CloseEntry")]
    pub close_entry: bool,
    
    /// Good Quantity in Warehouse Unit
    #[serde(rename = "QuantityGood")]
    pub quantity_good: i32,
    
    /// Issue information
    #[serde(rename = "IssueLines")]
    pub issue_lines: Vec<String>,
    
    /// receipt information
    #[serde(rename = "ReceiptLines")]
    pub receipt_lines: Vec<String>,
}
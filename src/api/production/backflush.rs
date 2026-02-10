#[derive(Debug, Clone, serde::Serialize)]
pub struct BackflushOptions
{
    /// Work oder number
    #[serde(rename = "DocEntry")]
    doc_entry: u32,
    
    /// Line Number 
    #[serde(rename = "LineNumber")]
    line_number: u32,
    
    /// Optional Document date, if not today.
    #[serde(rename = "DocDate")]
    doc_date: Option<f32>,
    
    /// set to true, if you want to close the work order
    #[serde(rename = "CloseEntry")]
    close_entry: bool,
    
    /// Good Quantity in Warehouse Unit
    #[serde(rename = "QuantityGood")]
    quantity_good: u32,
    
    /// Issue information
    #[serde(rename = "IssueLines")]
    issue_lines: Vec<String>,
    
    /// receipt information
    #[serde(rename = "ReceiptLines")]
    receipt_lines: Vec<String>,
}
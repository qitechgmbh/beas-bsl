#[derive(Debug, Clone)]
pub struct Filter
{
    pub data: String
}

#[derive(Debug, Clone)]
pub struct FilterBuilder
{
    pub data: String
}

#[derive(Debug, Clone)]
pub struct OperatorSelector
{
    builder: FilterBuilder
}

#[derive(Debug, Clone)]
pub struct LogicalOperatorSelector
{
    builder: FilterBuilder
}

impl FilterBuilder
{
    pub fn new() -> OperatorSelector
    {
        OperatorSelector { builder: Self { data: String::new() } }
    }
}

impl OperatorSelector
{
    pub fn equals<T: ToValue>(mut self, field: &str, value: T) -> LogicalOperatorSelector
    {
        let string = format!("{} eq {}", field, value.convert());
        self.builder.data.push_str(&string);

        LogicalOperatorSelector { builder: self.builder }
    }

    pub fn not_equals<T: ToValue>(mut self, field: &str, value: T) -> LogicalOperatorSelector
    {
        let string = format!("{} ne {}", field, value.convert());
        self.builder.data.push_str(&string);

        LogicalOperatorSelector { builder: self.builder }
    }

    pub fn greater_than<T: ToValue>(mut self, field: &str, value: T) -> LogicalOperatorSelector
    {
        let string = format!("{} gt {}", field, value.convert());
        self.builder.data.push_str(&string);

        LogicalOperatorSelector { builder: self.builder }
    }

    pub fn greater_than_or_equal<T: ToValue>(mut self, field: &str, value: T) -> LogicalOperatorSelector
    {
        let string = format!("{} ge {}", field, value.convert());
        self.builder.data.push_str(&string);

        LogicalOperatorSelector { builder: self.builder }
    }

    pub fn less_than<T: ToValue>(mut self, field: &str, value: T) -> LogicalOperatorSelector
    {
        let string = format!("{} lt {}", field, value.convert());
        self.builder.data.push_str(&string);

        LogicalOperatorSelector { builder: self.builder }
    }

    pub fn less_than_or_equal<T: ToValue>(mut self, field: &str, value: T) -> LogicalOperatorSelector
    {
        let string = format!("{} le {}", field, value.convert());
        self.builder.data.push_str(&string);

        LogicalOperatorSelector { builder: self.builder }
    }

    // non ODATA compliant beas-bsl string operators
    pub fn like(mut self, field: &str, value: &str) -> LogicalOperatorSelector
    {
        let escaped = value.replace('\'', "''");
        let string = format!("contains({}, '{}')", field, escaped);
        self.builder.data.push_str(string.as_str());
        
        LogicalOperatorSelector { builder: self.builder }
    }

    pub fn starts_with(mut self, field: &str, value: &str) -> LogicalOperatorSelector
    {
        let escaped = value.replace('\'', "''");
        let string = format!("startswith({}, '{}')", field, escaped);
        self.builder.data.push_str(string.as_str());
        
        LogicalOperatorSelector { builder: self.builder }
    }

    pub fn ends_with(mut self, field: &str, value: &str) -> LogicalOperatorSelector
    {
        let escaped = value.replace('\'', "''");
        let string = format!("endswith({}, '{}')", field, escaped);
        self.builder.data.push_str(string.as_str());
        
        LogicalOperatorSelector { builder: self.builder }
    }
}

impl LogicalOperatorSelector
{
    pub fn and(mut self) -> OperatorSelector
    {
        self.builder.data.push_str(" and ");
        return OperatorSelector { builder: self.builder }
    }

    pub fn or(mut self) -> OperatorSelector
    {
        self.builder.data.push_str(" or ");
        return OperatorSelector { builder: self.builder }
    }
    
    pub fn build(self) -> Filter
    {
        Filter { data: self.builder.data }
    }
}

pub trait ToValue
{
    fn convert(&self) -> String;
}

// Implement for allowed types
impl ToValue for &str
{
    fn convert(&self) -> String
    {
        format!("'{}'", self.replace('\'', "''")) // escape quotes
    }
}

impl ToValue for String
{
    fn convert(&self) -> String
    {
        self.as_str().to_string()
    }
}

impl ToValue for i64
{
    fn convert(&self) -> String
    {
        self.to_string()
    }
}

impl ToValue for bool
{
    fn convert(&self) -> String
    {
        self.to_string()
    }
}
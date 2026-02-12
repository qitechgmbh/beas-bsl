#[derive(Debug, Clone)]
pub struct Filter
{
    pub data: String
}

#[derive(Debug, Clone)]
pub struct OperatorSelector
{
    filter: Filter
}

#[derive(Debug, Clone)]
pub struct LogicalOperatorSelector
{
    filter: Filter
}

impl Filter
{
    pub fn new() -> OperatorSelector
    {
        OperatorSelector { filter: Self { data: String::new() } }
    }
}

impl OperatorSelector
{
    pub fn equals(mut self, field: &str, value: Value) -> LogicalOperatorSelector
    {
        let string = format!("{} eq {}", field, value);
        self.filter.data.push_str(string.as_str());
        
        LogicalOperatorSelector { filter: self.filter }
    }

    pub fn not_equals(mut self, field: &str, value: Value) -> LogicalOperatorSelector
    {
        let string = format!("{} ne {}", field, value);
        self.filter.data.push_str(string.as_str());
        
        LogicalOperatorSelector { filter: self.filter }
    }

    pub fn greater_than(mut self, field: &str, value: Value) -> LogicalOperatorSelector
    {
        let string = format!("{} gt {}", field, value);
        self.filter.data.push_str(string.as_str());
        
        LogicalOperatorSelector { filter: self.filter }
    }

    pub fn greater_than_or_equal(mut self, field: &str, value: Value) -> LogicalOperatorSelector
    {
        let string = format!("{} ge {}", field, value);
        self.filter.data.push_str(string.as_str());
        
        LogicalOperatorSelector { filter: self.filter }
    }

    pub fn less_than(mut self, field: &str, value: Value) -> LogicalOperatorSelector
    {
        let string = format!("{} lt {}", field, value);
        self.filter.data.push_str(string.as_str());
        
        LogicalOperatorSelector { filter: self.filter }
    }

    pub fn less_than_or_equal(mut self, field: &str, value: Value) -> LogicalOperatorSelector
    {
        let string = format!("{} le {}", field, value);
        self.filter.data.push_str(string.as_str());
        
        LogicalOperatorSelector { filter: self.filter }
    }

    pub fn like(mut self, field: &str, value: &str) -> LogicalOperatorSelector
    {
        let escaped = value.replace('\'', "''");
        let string = format!("contains({}, '{}')", field, escaped);
        self.filter.data.push_str(string.as_str());
        
        LogicalOperatorSelector { filter: self.filter }
    }

    pub fn starts_with(mut self, field: &str, value: &str) -> LogicalOperatorSelector
    {
        let escaped = value.replace('\'', "''");
        let string = format!("startswith({}, '{}')", field, escaped);
        self.filter.data.push_str(string.as_str());
        
        LogicalOperatorSelector { filter: self.filter }
    }

    pub fn ends_with(mut self, field: &str, value: &str) -> LogicalOperatorSelector
    {
        let escaped = value.replace('\'', "''");
        let string = format!("endswith({}, '{}')", field, escaped);
        self.filter.data.push_str(string.as_str());
        
        LogicalOperatorSelector { filter: self.filter }
    }
}

impl From<OperatorSelector> for Filter
{
    fn from(selector: OperatorSelector) -> Self
    {
        selector.filter
    }
}

impl LogicalOperatorSelector
{
    pub fn and(mut self) -> OperatorSelector
    {
        self.filter.data.push_str(" and ");
        return OperatorSelector { filter: self.filter }
    }

    pub fn or(mut self) -> OperatorSelector
    {
        self.filter.data.push_str(" or ");
        return OperatorSelector { filter: self.filter }
    }
}

pub enum Value
{
    String(String),
    Number(i64),
    Bool(bool),
}

impl std::fmt::Display for Value
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Value::String(s) => write!(f, "'{}'", s.replace('\'', "''")),
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}
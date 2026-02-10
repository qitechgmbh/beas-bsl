#[derive(Debug, Clone, Default)]
pub struct QueryOptions
{
    count:    bool,
    top:      Option<u64>,
    skip:     Option<u64>,
    order_by: Vec<OrderByClause>,
    filters:  Vec<Filter>,
    select:   Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Query
{
    pub param: &'static str,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct OrderByClause
{
    pub field:    String,
    pub ordering: Ordering,
}

#[derive(Debug, Clone)]
pub struct Filter
{
    pub lhs: String,
    pub op:  FilterOperator,
    pub rhs: String,
}

#[derive(Debug, Clone)]
pub enum FilterOperator
{
    // Default ops
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    
    // Additional operators for strings that are not compatible with the ODATA syntax:
    Like,
    StartWith,
    EndWith,
}

#[derive(Debug, Clone)]
pub enum Ordering 
{
    Ascending,
    Descending,
}

impl QueryOptions
{
    pub fn new() -> Self
    {
        Self::default()
    }

    pub fn count(mut self) -> Self
    {
        self.count = true;
        self
    }

    pub fn top(mut self, value: u64) -> Self
    {
        self.top = Some(value);
        self
    }

    pub fn skip(mut self, value: u64) -> Self
    {
        self.skip = Some(value);
        self
    }

    pub fn order_by(mut self, field: &str, ordering: Ordering) -> Self
    {
        self.order_by.push(OrderByClause { field: field.to_string(), ordering });
        self
    }

    pub fn filter<L, R>(mut self, lhs: L, op: FilterOperator, rhs: R) -> Self
    where
        L: Into<String>,
        R: Into<String>,
    {
        self.filters.push(Filter
        {
            lhs: lhs.into(),
            op,
            rhs: rhs.into(),
        });
        self
    }

    // pub fn select(mut self, field: &'static str) -> Self
    // {
    //     self.select.push(field.to_string());
    //     self
    // }

    pub fn select<F, I>(mut self, fields: I) -> Self
    where
        F: AsRef<str>,
        I: IntoIterator<Item = F>,
    {
        self.select.clear();
        self.select.extend(fields.into_iter().map(|f| f.as_ref().to_string()));
        self
    }

    pub fn to_query_list(&self) -> Vec<Query>
    {
        let mut result: Vec<Query> = Default::default();
        
        if self.count
        {
            result.push(Query { 
                param: "$count", 
                value: "".to_string() 
            });
        }

        if let Some(top) = self.top
        {
            result.push(Query { 
                param: "$top", 
                value: format!("{}", top)
            });
        }

        if let Some(skip) = self.skip
        {
            result.push(Query { 
                param: "$skip", 
                value: format!("{}", skip)
            });
        }

        if !self.order_by.is_empty()
        {
            let mut value = String::new();

            for (i, clause) in self.order_by.iter().enumerate()
            {
                if i > 0
                {
                    value.push_str(", ");
                }
                
                value.push_str(&clause.field);
                
                match clause.ordering
                {
                    Ordering::Ascending  => value.push_str(" asc"),
                    Ordering::Descending => value.push_str(" desc"),
                }
            }

            result.push(Query { param: "$orderby", value });
        }

        if !self.filters.is_empty()
        {
            let mut value = String::new();

            for (i, filter) in self.filters.iter().enumerate()
            {
                if i > 0
                {
                    value.push_str(" and ");
                }
                
                value.push_str(&filter.to_string());
            }

            result.push(Query { param: "$filter", value });
        }
        
        result
    }
}

impl ToString for Filter
{
    fn to_string(&self) -> String
    {
        format!("{} {} {}", self.lhs, self.op.to_string(), self.rhs)
    }
}

impl ToString for FilterOperator
{
    fn to_string(&self) -> String
    {
        match self
        {
            FilterOperator::Equal => "eq".into(),
            FilterOperator::NotEqual => "ne".into(),
            FilterOperator::GreaterThan => "gt".into(),
            FilterOperator::GreaterThanOrEqual => "ge".into(),
            FilterOperator::LessThan => "lt".into(),
            FilterOperator::LessThanOrEqual => "le".into(),
            FilterOperator::Like => "like".into(),
            FilterOperator::StartWith => "startswith".into(),
            FilterOperator::EndWith => "endswith".into(),
        }
    }
}
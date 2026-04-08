/// Represents a query filter string, e.g., for OData-style `$filter`.
#[derive(Debug, Clone)]
pub struct Filter {
    pub data: String,
}

/// Internal builder for constructing a filter.
#[derive(Debug, Clone)]
pub struct FilterBuilder {
    pub data: String,
}

/// Selector for choosing a comparison operator.
#[derive(Debug, Clone)]
pub struct OperatorSelector {
    builder: FilterBuilder,
}

/// Selector for adding logical operators between conditions.
#[derive(Debug, Clone)]
pub struct LogicalOperatorSelector {
    builder: FilterBuilder,
}

impl FilterBuilder {
    /// Starts building a new filter.
    pub fn new() -> OperatorSelector {
        OperatorSelector {
            builder: Self {
                data: String::new(),
            },
        }
    }
}

impl OperatorSelector {
    /// Field equals value.
    pub fn equals<T: ToValue>(mut self, field: &str, value: T) -> LogicalOperatorSelector {
        let string = format!("{} eq {}", field, value.convert());
        self.builder.data.push_str(&string);

        LogicalOperatorSelector {
            builder: self.builder,
        }
    }

    /// Field not equals value.
    pub fn not_equals<T: ToValue>(mut self, field: &str, value: T) -> LogicalOperatorSelector {
        let string = format!("{} ne {}", field, value.convert());
        self.builder.data.push_str(&string);

        LogicalOperatorSelector {
            builder: self.builder,
        }
    }

    /// Field greater than value.
    pub fn greater_than<T: ToValue>(mut self, field: &str, value: T) -> LogicalOperatorSelector {
        let string = format!("{} gt {}", field, value.convert());
        self.builder.data.push_str(&string);

        LogicalOperatorSelector {
            builder: self.builder,
        }
    }

    /// Field greater than or equal to value.
    pub fn greater_than_or_equal<T: ToValue>(
        mut self,
        field: &str,
        value: T,
    ) -> LogicalOperatorSelector {
        let string = format!("{} ge {}", field, value.convert());
        self.builder.data.push_str(&string);

        LogicalOperatorSelector {
            builder: self.builder,
        }
    }

    /// Field less than value.
    pub fn less_than<T: ToValue>(mut self, field: &str, value: T) -> LogicalOperatorSelector {
        let string = format!("{} lt {}", field, value.convert());
        self.builder.data.push_str(&string);

        LogicalOperatorSelector {
            builder: self.builder,
        }
    }

    /// Field less than or equal to value.
    pub fn less_than_or_equal<T: ToValue>(
        mut self,
        field: &str,
        value: T,
    ) -> LogicalOperatorSelector {
        let string = format!("{} le {}", field, value.convert());
        self.builder.data.push_str(&string);

        LogicalOperatorSelector {
            builder: self.builder,
        }
    }

    /// Field contains string (non-ODATA compliant).
    pub fn like(mut self, field: &str, value: &str) -> LogicalOperatorSelector {
        let escaped = value.replace('\'', "''");
        let string = format!("contains({}, '{}')", field, escaped);
        self.builder.data.push_str(&string);

        LogicalOperatorSelector {
            builder: self.builder,
        }
    }

    /// Field starts with string.
    pub fn starts_with(mut self, field: &str, value: &str) -> LogicalOperatorSelector {
        let escaped = value.replace('\'', "''");
        let string = format!("startswith({}, '{}')", field, escaped);
        self.builder.data.push_str(&string);

        LogicalOperatorSelector {
            builder: self.builder,
        }
    }

    /// Field ends with string.
    pub fn ends_with(mut self, field: &str, value: &str) -> LogicalOperatorSelector {
        let escaped = value.replace('\'', "''");
        let string = format!("endswith({}, '{}')", field, escaped);
        self.builder.data.push_str(&string);

        LogicalOperatorSelector {
            builder: self.builder,
        }
    }
}

impl LogicalOperatorSelector {
    /// Chain another condition with `and`.
    pub fn and(mut self) -> OperatorSelector {
        self.builder.data.push_str(" and ");

        OperatorSelector {
            builder: self.builder,
        }
    }

    /// Chain another condition with `or`.
    pub fn or(mut self) -> OperatorSelector {
        self.builder.data.push_str(" or ");

        OperatorSelector {
            builder: self.builder,
        }
    }

    /// Finalizes and returns the filter string.
    pub fn build(self) -> Filter {
        Filter {
            data: self.builder.data,
        }
    }
}

// To Value helper
pub trait ToValue {
    fn convert(&self) -> String;
}

impl ToValue for &str {
    fn convert(&self) -> String {
        format!("'{}'", self.replace('\'', "''")) // escape quotes
    }
}

impl ToValue for String {
    fn convert(&self) -> String {
        self.as_str().to_string()
    }
}

impl ToValue for f64 {
    fn convert(&self) -> String {
        self.to_string()
    }
}
impl ToValue for f32 {
    fn convert(&self) -> String {
        self.to_string()
    }
}

impl ToValue for i64 {
    fn convert(&self) -> String {
        self.to_string()
    }
}
impl ToValue for i32 {
    fn convert(&self) -> String {
        self.to_string()
    }
}
impl ToValue for i16 {
    fn convert(&self) -> String {
        self.to_string()
    }
}
impl ToValue for i8 {
    fn convert(&self) -> String {
        self.to_string()
    }
}

impl ToValue for u64 {
    fn convert(&self) -> String {
        self.to_string()
    }
}
impl ToValue for u32 {
    fn convert(&self) -> String {
        self.to_string()
    }
}
impl ToValue for u16 {
    fn convert(&self) -> String {
        self.to_string()
    }
}
impl ToValue for u8 {
    fn convert(&self) -> String {
        self.to_string()
    }
}

impl ToValue for bool {
    fn convert(&self) -> String {
        self.to_string()
    }
}

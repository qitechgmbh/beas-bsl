use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Error
{
    pub code:    u32,
    pub message: ErrorMessage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorMessage
{
    pub lang:  String,
    pub value: String,
}

impl std::fmt::Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "[{}] {}", self.code, self.message.value)
    }
}
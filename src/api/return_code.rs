#[derive(Debug, Clone)]
pub enum ReturnCode
{
    /// A GET or function call request returns 200 OK if 
    /// the operation is completed successfully. 
    /// The response body must contain the value of the entity or property specified in the request URL.
    GetOk,
    
    /// The request is successfully executed by POST/ PUT /DELETE method
    MutOk,
    
    /// Not supported.
    NotSupported(u32),
    
    /// Syntax error in URI String
    SyntaxError,
    
    /// Login fail, wrong password (Login fail) or
    /// Authentication is required and has not been provided. (Unauthorized)
    LoginFail,
    
    // Can be:
    // - Entity not found
    // - Function not found
    // - Resource (the Data record, defined in bracket) not found
    NotFound,
    
    /// A request cannot be used for this record or login not possible with this request type.
    MethodNotAllowed,
    
    /// Internal error (SQL error, program error)
    InternalError,
    
    /// Not implemented
    NotImplemented,
    
    /// Unknown
    Unknown(u32),
}

impl ReturnCode
{
    pub fn from_int(code: u32) -> Self  
    {
        match code 
        {
            200 => ReturnCode::GetOk,
            201 => ReturnCode::MutOk,
            400 => ReturnCode::SyntaxError,
            401 => ReturnCode::LoginFail,
            404 => ReturnCode::NotFound,
            405 => ReturnCode::MethodNotAllowed,
            500 => ReturnCode::InternalError,
            501 => ReturnCode::NotImplemented,
            300..=399 => ReturnCode::NotSupported(code),
            other => ReturnCode::Unknown(other),
        }
    }
}
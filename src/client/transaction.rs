


#[derive(Debug, Clone)]
pub struct TransactionHandle {
    uid:          u64,
    request_type: Type,
    endpoint:     Endpoint,
}

#[derive(Debug, Clone)]
pub enum Type {
    Get,
    Post,
}

#[derive(Debug, Clone)]
pub enum Endpoint {
    Production(ProductionEndpoint)
}

#[derive(Debug, Clone)]
pub enum ProductionEndpoint {
    
}
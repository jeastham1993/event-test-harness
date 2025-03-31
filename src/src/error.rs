use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("DynamoDB error: {0}")]
    DynamoDb(#[from] aws_sdk_dynamodb::Error),
    
    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),
    
    #[error("Invalid event data: {0}")]
    InvalidEvent(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Runtime error: {0}")]
    Runtime(String),
}

pub type Result<T> = std::result::Result<T, Error>; 
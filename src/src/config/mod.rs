use serde::Deserialize;
use crate::error::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub table_name: String,
    pub property_path: String,
    pub environment: Environment,
    pub event_source: EventSource,
}

#[derive(Debug, Deserialize)]
pub enum Environment {
    Local,
    Lambda,
    Container,
}

#[derive(Debug, Deserialize)]
pub enum EventSource {
    Sns,
    Sqs,
    EventBridge,
    Http,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let environment = match std::env::var("EVENT_HARNESS_LAMBDA") {
            Ok(_) => Environment::Lambda,
            Err(_) => Environment::Container,
        };

        let event_source = match std::env::var("EVENT_SOURCE") {
            Ok(val) => match val.to_lowercase().as_str() {
                "sqs" => EventSource::Sqs,
                "eventbridge" => EventSource::EventBridge,
                "http" => EventSource::Http,
                _ => return Err(Error::Config("Invalid EVENT_SOURCE value. Must be 'sqs' or 'eventbridge'".to_string())),
            },
            Err(_) => EventSource::Sqs, // Default to SQS for backward compatibility
        };

        let table_name = std::env::var("TABLE_NAME")
            .map_err(|e| Error::Config(format!("TABLE_NAME environment variable not set: {}", e)))?;

        let property_path = std::env::var("EVENT_HARNESS_PROPERTY_JSON_POINTER_PATH")
            .map_err(|e| Error::Config(format!("EVENT_HARNESS_PROPERTY_JSON_POINTER_PATH environment variable not set: {}", e)))?;

        Ok(Self {
            table_name,
            property_path,
            environment,
            event_source,
        })
    }
} 
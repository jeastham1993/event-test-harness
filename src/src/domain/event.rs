use std::time::SystemTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub timestamp: SystemTime,
    pub contents: String,
    pub property_value: String,
    pub event_type: String
}

impl Event {
    pub fn new(event_type: String, contents: String, property_value: String) -> Self {
        Self {
            timestamp: SystemTime::now(),
            contents,
            property_value,
            event_type
        }
    }
    
    pub(crate) fn from(timestamp: SystemTime, event_type: String, contents: String, property_value: String) -> Self {
        Self {
            timestamp,
            contents,
            property_value,
            event_type
        }
    }
} 
use std::sync::Arc;
use crate::error::{Error};
use crate::domain::{Event, EventRepository};
use serde_json::Value;

pub struct EventService {
    repository: Arc<dyn EventRepository>,
}

impl EventService {
    pub fn new(repository: Arc<dyn EventRepository>) -> Self {
        Self { repository }
    }

    pub async fn process_event(&self, event_type: &str, event_data: &str, property_path: &str) -> Result<(), Error> {
        let json_value = serde_json::from_str::<Value>(event_data)
            .map_err(|e| Error::JsonParse(e))?;

        let property_value = json_value
            .pointer(property_path)
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::InvalidEvent(format!("Property path {} not found", property_path)))?
            .to_string();

        let event = Event::new(event_type.to_string(), event_data.to_string(), property_value);
        self.repository.save(&event).await
    }

    pub async fn find_by_id(&self, identifier: &str) -> Result<Vec<Event>, Error> {
        self.repository.find_by_id(identifier).await
    }
} 
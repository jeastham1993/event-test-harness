use std::sync::Arc;
use aws_lambda_events::cloudwatch_events::CloudWatchEvent;
use lambda_runtime::{run, service_fn, LambdaEvent};
use crate::error::{Error};
use crate::application::EventService;

pub struct EventBridgeHandler {
    event_service: Arc<EventService>,
    property_path: String,
}

impl EventBridgeHandler {
    pub fn new(event_service: Arc<EventService>, property_path: String) -> Self {
        Self {
            event_service,
            property_path,
        }
    }

    pub async fn run(&self) {
        let _ = run(service_fn(|event| self.handle_event(event))).await;
    }

    async fn handle_event(&self, event: LambdaEvent<CloudWatchEvent<serde_json::Value>>) -> Result<(), lambda_runtime::Error> {
        let detail = event.payload.detail;
        let event_data = serde_json::to_string(&detail)
            .map_err(|e| lambda_runtime::Error::from(Error::JsonParse(e)))?;

        self.event_service
            .process_event(&event.payload.detail_type.unwrap_or("".to_string()), &event_data, &self.property_path)
            .await
            .map_err(|e| lambda_runtime::Error::from(e))?;

        Ok(())
    }
} 
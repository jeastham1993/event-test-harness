use std::sync::Arc;
use aws_lambda_events::sns::SnsEvent;
use lambda_runtime::{run, service_fn, LambdaEvent};
use crate::application::EventService;

pub struct SnsHandler {
    event_service: Arc<EventService>,
    property_path: String,
}

impl SnsHandler {
    pub fn new(event_service: Arc<EventService>, property_path: String) -> Self {
        Self {
            event_service,
            property_path,
        }
    }

    pub async fn run(&self) {
        let _ = run(service_fn(|event| self.handle_event(event))).await;
    }

    async fn handle_event(&self, event: LambdaEvent<SnsEvent>) -> Result<(), lambda_runtime::Error> {
        for record in event.payload.records {
            self.event_service
                .process_event(&record.sns.topic_arn, &record.sns.message, &self.property_path)
                .await
                .map_err(|e| lambda_runtime::Error::from(e))?;
        }
        Ok(())
    }
} 
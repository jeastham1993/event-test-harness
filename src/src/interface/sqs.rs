use std::sync::Arc;
use aws_lambda_events::sqs::SqsEvent;
use lambda_runtime::{run, service_fn, LambdaEvent};
use crate::application::EventService;

pub struct SqsHandler {
    event_service: Arc<EventService>,
    property_path: String,
}

impl SqsHandler {
    pub fn new(event_service: Arc<EventService>, property_path: String) -> Self {
        Self {
            event_service,
            property_path,
        }
    }

    pub async fn run(&self) {
        let _ = run(service_fn(|event| self.handle_event(event))).await;
    }

    async fn handle_event(&self, event: LambdaEvent<SqsEvent>) -> Result<(), lambda_runtime::Error> {
        for record in event.payload.records {
            if let Some(body) = record.body {
                self.event_service
                    .process_event(&record.event_source_arn.unwrap_or("".to_string()), &body, &self.property_path)
                    .await
                    .map_err(|e| lambda_runtime::Error::from(e))?;
            }
        }
        Ok(())
    }
} 
use std::sync::Arc;
use lambda_http::{run, service_fn, Body, tracing::{self}, Error, IntoResponse, Request, RequestExt, Response};
use lambda_http::http::{StatusCode};
use crate::application::EventService;

pub struct HttpHandler {
    event_service: Arc<EventService>,
}

impl HttpHandler {
    pub fn new(event_service: Arc<EventService>) -> Self {
        Self {
            event_service,
        }
    }

    pub async fn run(&self) {
        let _ = run(service_fn(|event| self.handle_event(event))).await;
    }

    async fn handle_event(&self, event: Request) -> Result<impl IntoResponse, Error> {
        tracing::info!("Received event: {:?}", event);

        let identifier = event
            .path_parameters_ref()
            .and_then(|params| params.first("identifier"))
            .unwrap_or("");

        let response = &self.event_service.find_by_id(&identifier).await.unwrap();

        let response = Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "Content-Type")
            .header("Access-Control-Allow-Methods", "POST,GET,PUT,DELETE")
            .body(Body::Text(
                serde_json::to_string(&response).unwrap_or("".to_string()),
            ))
            .map_err(Box::new)?;

        Ok(response)
    }
} 
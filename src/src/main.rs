mod domain;
mod application;
mod infrastructure;
mod interface;
mod config;
mod error;

use std::sync::Arc;
use aws_config::BehaviorVersion;
use lambda_http::tracing::subscriber::EnvFilter;
use tracing::{info, level_filters::LevelFilter};
use crate::application::EventService;
use crate::config::{Config, Environment, EventSource};
use crate::infrastructure::DynamoDbRepository;
use crate::interface::{EventBridgeHandler, SqsHandler};
use crate::interface::container::{ContainerRunner, TestConfig};
use crate::interface::http::HttpHandler;
use crate::interface::sns::SnsHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup tracing subscriber for logging
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(filter).without_time().init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load config from environment");

    tracing::log::info!("Configuration loaded: {:?}", config);

    match config.environment {
        Environment::Local => {
            info!("Starting local runner using container configuration");

            let test_config = std::env::var("TEST_CONFIG").expect("Environment variable 'TEST_CONFIG' not found");
            let test_config: TestConfig = serde_json::from_str(&test_config).expect("Failed to parse TEST_CONFIG");
            let container_runner = ContainerRunner::new(test_config);

            container_runner.run().await;
        },
        Environment::Lambda => {
            // Initialize AWS clients
            let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
            let dynamo_db_client = aws_sdk_dynamodb::Client::new(&aws_config);

            // Initialize repositories
            let repository = Arc::new(DynamoDbRepository::new(
                dynamo_db_client,
                config.table_name.clone(),
            ));

            tracing::log::info!("Repository initialized");

            // Initialize application services
            let event_service = Arc::new(EventService::new(repository));

            info!("Starting lambda handler");

            match config.event_source {
                EventSource::Sns => {
                    info!("Starting SNS event handler");

                    let sns_handler = SnsHandler::new(event_service, config.property_path);
                    sns_handler.run().await;
                }
                EventSource::Sqs => {
                    info!("Starting SQS event handler");

                    let sqs_handler = SqsHandler::new(event_service, config.property_path);
                    sqs_handler.run().await;
                }
                EventSource::EventBridge => {
                    info!("Starting event bridge handler");

                    let event_bridge_handler = EventBridgeHandler::new(event_service, config.property_path);
                    event_bridge_handler.run().await;
                }
                EventSource::Http => {
                    info!("Starting HTTP event handler");

                    let http_handler = HttpHandler::new(event_service);
                    http_handler.run().await;
                }
            }
        },
        Environment::Container => {
            info!("Starting container runner");
            
            let test_config = std::env::var("TEST_CONFIG").expect("Environment variable 'TEST_CONFIG' not found");
            let test_config: TestConfig = serde_json::from_str(&test_config).expect("Failed to parse TEST_CONFIG");
            let container_runner = ContainerRunner::new(test_config);
            
            container_runner.run().await;
        }
    }
    
    Ok(())
}

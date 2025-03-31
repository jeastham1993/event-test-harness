use std::collections::HashMap;
use std::time::SystemTime;
use crate::domain::{Event, EventRepository};
use crate::error::Error;
use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

pub struct DynamoDbRepository {
    client: Client,
    table_name: String,
}

impl DynamoDbRepository {
    pub fn new(client: Client, table_name: String) -> Self {
        Self { client, table_name }
    }
}

#[async_trait]
impl EventRepository for DynamoDbRepository {
    async fn save(&self, event: &Event) -> Result<(), Error> {
        let timestamp = event
            .timestamp
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| Error::Runtime(format!("Invalid timestamp: {}", e)))?
            .as_millis()
            .to_string();

        let put_res = self.client
            .put_item()
            .table_name(&self.table_name)
            .item("PK", AttributeValue::S(event.property_value.clone()))
            .item("SK", AttributeValue::S(timestamp))
            .item("contents", AttributeValue::S(event.contents.clone()))
            .item("eventType", AttributeValue::S(event.event_type.clone()))
            .send()
            .await;
        
        match put_res {
            Ok(_) => Ok(()),
            Err(e) => {
                Err(Error::Runtime(format!("Failed to save event: {}", e)))
            }
        }
    }

    async fn find_by_id(&self, identifier: &str) -> Result<Vec<Event>, Error> {
        let query_res = &self
            .client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("PK = :pk")
            .expression_attribute_values(":pk", AttributeValue::S(identifier.to_string()))
            .send()
            .await;

        let mut response_contents: Vec<Event> = vec![];

        match query_res {
            Ok(output) => match &output.items {
                None => {}
                Some(items) => {
                    for item in items {
                        response_contents.push(item.into());
                    }
                }
            },
            Err(e) => {
                println!("Error querying DynamoDB: {:?}", e);
            }
        }
        Ok(response_contents)
    }
}

impl From<&HashMap<String, AttributeValue>> for Event {
    fn from(value: &HashMap<String, AttributeValue>) -> Self {
        let timestamp_str = value.get("SK").unwrap().as_s().unwrap();
        let timestamp = string_to_system_time(timestamp_str).unwrap();
        
        Event::from(
            timestamp,
            value.get("eventType").unwrap().as_s().unwrap().to_string(),
            value.get("contents").unwrap().as_s().unwrap().to_string(),
            value.get("PK").unwrap().as_s().unwrap().to_string(),
        )
    }
}

fn string_to_system_time(timestamp_str: &str) -> Result<SystemTime, Error> {
    // Parse the string to u128 (milliseconds)
    let millis = timestamp_str.parse::<u128>()
        .map_err(|e| Error::Runtime(format!("Invalid timestamp string: {}", e)))?;

    // Convert milliseconds to Duration
    // Note: from_millis takes u64, so we need to handle potential overflow
    if millis > u64::MAX as u128 {
        return Err(Error::Runtime("Timestamp value too large".to_string()));
    }

    let duration = std::time::Duration::from_millis(millis as u64);

    // Add duration to UNIX_EPOCH
    Ok(std::time::UNIX_EPOCH + duration)
}

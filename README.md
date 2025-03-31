# Rust Event Harness

A generic event harness for testing producers inn event-driven architectures.

## Why?

## Features

- Process events from multiple sources:
  - Amazon SQS
  - Amazon SNS
  - Amazon EventBridge
- Store events in DynamoDB
- Configurable JSON property extraction
- Support for both Lambda and container environments

## Configuration

Required environment variables:

- `TABLE_NAME`: DynamoDB table name
- `EVENT_HARNESS_PROPERTY_JSON_POINTER_PATH`: JSON pointer path to extract property value
- `EVENT_SOURCE`: (Optional) Event source type - "sqs" or "eventbridge" (defaults to "sqs")

### Event Source Examples

#### SQS Event
```json
{
  "test": {
    "nested": "value"
  }
}
```

#### EventBridge Event
```json
{
  "detail": {
    "test": {
      "nested": "value"
    }
  }
}
```
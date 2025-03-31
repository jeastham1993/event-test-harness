use std::sync::Arc;
use aws_config::BehaviorVersion;
use aws_sdk_sqs::Client;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TestConfig {
    channels_under_test: Vec<ChannelUnderTest>,
}

#[derive(Deserialize, Serialize)]
struct ChannelUnderTest {
    channel_type: Channels,
}

#[derive(Deserialize, Serialize)]
enum Channels {
    SQS{queue_url: String},
}

struct SqsConfig {
    queue_url: String,
}

impl SqsConfig {
    async fn receive_message(&mut self, client: &Client, received_messages: &Arc<Mutex<Vec<String>>>) {
        loop {
            let rcv_message_output = client.receive_message().queue_url(&self.queue_url).send().await.unwrap();

            println!("Messages from queue with url: {}", &self.queue_url);

            for message in rcv_message_output.messages.unwrap_or_default() {
                println!("Got the message: {:#?}", message);

                let mut messages = received_messages.lock().await;
                messages.push(message.body.unwrap().clone());

                client.delete_message().queue_url(&self.queue_url).receipt_handle(message.receipt_handle.unwrap()).send().await.unwrap();
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
}

pub  struct ContainerRunner {
    config: TestConfig,
}

impl ContainerRunner {
    pub fn new(test_config: TestConfig) -> Self {
        Self {
            config: test_config,
        }
    }
    
    pub async fn run(&self) {
        let received_messages: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

        for x in &self.config.channels_under_test {
            match &x.channel_type {
                Channels::SQS{queue_url} => {
                    let received_messages_clone = Arc::clone(&received_messages);
                    let queue_url = queue_url.clone();
                    
                    tokio::spawn(async move {
                        let mut sqs_config = SqsConfig {
                            queue_url,
                        };
                        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
                        let client = aws_sdk_sqs::Client::new(&config);
                        sqs_config.receive_message(&client, &received_messages_clone).await;
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    });       
                }
            }
        }

        loop {
            println!("Main thread");

            tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
        }
    }
}
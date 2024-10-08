use async_trait::async_trait;
use aws_sdk_sqs::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tokio::time::Duration;

pub struct SqsListenerBuilder {
    client: Client,
    listeners: HashMap<String, Arc<dyn SqsListener + Sync + Send>>,
    delay: Duration,
}

#[async_trait]
pub trait SqsListener {
    async fn on_message_received(&self, message: String) -> ();
}


impl SqsListenerBuilder {
    pub async fn from(client: Client) -> SqsListenerBuilder {
        SqsListenerBuilder { client, listeners: HashMap::new(), delay: Duration::from_millis(300) }
    }

    pub fn polling_delay(mut self, delay: Duration) -> SqsListenerBuilder {
        self.delay = delay;
        self
    }

    pub fn add_queue(mut self, queue_url: String, consumer: Arc<dyn SqsListener + Sync + Send>) -> SqsListenerBuilder {
        self.listeners.insert(queue_url, consumer);
        self
    }

    pub fn run(self) -> JoinHandle<()> {
        tokio::task::spawn(async move {
            let listeners = self.listeners;
            let client = self.client;
            let delay = self.delay;
            loop {
                for (queue_url, consumer) in &listeners {
                    SqsListenerBuilder::receive(&client, queue_url, &consumer).await;
                }
                sleep(delay).await;
            }
        })
    }

    async fn receive(client: &Client, queue_url: &String, consumer: &Arc<dyn SqsListener + Sync + Send>) {
        let rcv_message_output = client.receive_message()
            .queue_url(queue_url)
            .send().await.unwrap();

        for message in rcv_message_output.messages.unwrap_or_default() {
            let body = message.body.unwrap_or_default();

            tracing::info!("Received message from queue '{}' : {}", queue_url, body);

            consumer.on_message_received(body).await;

            client.delete_message()
                .queue_url(queue_url)
                .receipt_handle(message.receipt_handle.unwrap())
                .send()
                .await.unwrap();
        }
    }
}
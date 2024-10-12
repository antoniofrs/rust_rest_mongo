use async_trait::async_trait;
use aws_sdk_sqs::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinSet;
use tokio::time::sleep;
use tokio::time::Duration;

pub struct SqsListenerBuilder {
    client: Arc<Client>,
    listeners: HashMap<String, Arc<dyn SqsListener + Sync + Send>>,
    delay: u64,
}

#[async_trait]
pub trait SqsListener {
    async fn on_message_received(&self, message: String) -> Result<(), ()>;
}


impl SqsListenerBuilder {
    pub async fn from(client: Arc<Client>) -> SqsListenerBuilder {
        SqsListenerBuilder { client, listeners: HashMap::new(), delay: 300 }
    }

    pub fn polling_delay(mut self, delay: u64) -> SqsListenerBuilder {
        self.delay = delay;
        self
    }

    pub fn add_queue(mut self, queue_url: String, consumer: Arc<dyn SqsListener + Sync + Send>) -> SqsListenerBuilder {
        self.listeners.insert(queue_url, consumer);
        self
    }

    pub async fn run(self) {
        let listeners = self.listeners;
        let client = self.client;
        let delay = self.delay;

        let mut join_set = JoinSet::new();

        for (queue_url, consumer) in listeners {
            let client_clone = Arc::clone(&client);
            join_set.spawn(async move {
                loop {
                    receive_queue_message(&client_clone, &queue_url, &consumer).await;
                    sleep(Duration::from_millis(delay)).await;
                }
            });
        }

        while let Some(_) = join_set.join_next().await {}
    }
}

async fn receive_queue_message(
    client: &Client,
    queue_url: &String,
    consumer: &Arc<dyn SqsListener + Sync + Send>,
) {
    let rcv_message_output = client.receive_message()
        .queue_url(queue_url)
        .send().await.unwrap();

    for message in rcv_message_output.messages.unwrap_or_default() {
        let body = message.body.unwrap_or_default();

        if consumer.on_message_received(body).await.is_ok() {
            client.delete_message()
                .queue_url(queue_url)
                .receipt_handle(message.receipt_handle.unwrap())
                .send()
                .await
                .unwrap();
        }
    }
}
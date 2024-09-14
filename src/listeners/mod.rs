use aws_config::retry::RetryConfig;
use aws_sdk_sqs::config::Credentials;
use aws_sdk_sqs::Client;
use tokio::time::sleep;
use tokio::time::Duration;
// awslocal sqs send-message --queue-url http://sqs.eu-central-1.localhost.localstack.cloud:4566/000000000000/default --message-body '{ "message": "$(date +%s)-test" }'


pub async fn sqs_client() -> Client {
    let credentials = Credentials::new(
        "test",
        "test",
        Some("session-token".to_string()),
        None,
        "test",
    );

    let config = aws_config::from_env()
        .retry_config(RetryConfig::adaptive())
        .endpoint_url("http://localhost:4566")
        .region("us-east-1")
        .credentials_provider(credentials)
        .load().await;

    Client::new(&config)
}

pub async fn init_sqs_listener() {
    let client = sqs_client().await;
    let queue_url = String::from(
        "http://sqs.eu-central-1.localhost.localstack.cloud:4566/000000000000/default"
    );

    tracing::info!("Queue listener initialized");

    loop {
        receive(&client, &queue_url).await;
        sleep(Duration::from_millis(1000)).await;
    }
}

async fn receive(client: &Client, queue_url: &String) {
    let rcv_message_output = client.receive_message()
        .queue_url(queue_url)
        .send().await.unwrap();

    for message in rcv_message_output.messages.unwrap_or_default() {
        tracing::info!("Got the message: {:#?}", message.body.unwrap_or_default());

        client.delete_message()
            .queue_url(queue_url)
            .receipt_handle(message.receipt_handle.unwrap())
            .send()
            .await.unwrap();
    }
}
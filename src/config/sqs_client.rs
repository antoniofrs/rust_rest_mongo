use aws_config::retry::RetryConfig;
use aws_sdk_sqs::config::Credentials;
use aws_sdk_sqs::Client;


pub async fn get_local_sqs_client() -> Client {
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


pub async fn get_sqs_client() -> Client {
    let config = aws_config::load_from_env().await;
    Client::new(&config)
}
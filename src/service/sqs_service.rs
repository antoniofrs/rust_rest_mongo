use std::sync::Arc;
use crate::error_handler::internal_server_exception::queue_error;
use crate::error_handler::model::app_error::AppError;
use aws_sdk_sqs::Client;
use axum::extract::FromRef;


#[derive(Clone, FromRef)]
pub struct SqsService {
    client: Arc<Client>,
}

impl SqsService {
    pub fn new(client: Arc<Client>) -> Self {
        SqsService { client }
    }

    pub async fn send(&self, queue_url: String, message: String) -> Result<(), AppError> {

        self.client
            .send_message()
            .queue_url(queue_url)
            .message_body(&message)
            .send()
            .await
            .map_err(|_| { queue_error() })?;

        Ok(())
    }
}
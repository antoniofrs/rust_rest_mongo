use crate::error_handler::model::app_error::AppError;
use crate::model::sqs_queue::queue_message::QueueMessageRequest;
use crate::service::sqs_service::SqsService;
use axum::extract::State;
use axum::Json;

pub async fn queue_producer(
    State(queue_service): State<SqsService>,
    Json(message_request): Json<QueueMessageRequest>
) -> Result<(), AppError> {
    queue_service.send(message_request.queue, message_request.message).await?;
    Ok(())
}
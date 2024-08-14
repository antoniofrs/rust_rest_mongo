use crate::error_handler::model::app_error::{AppError, ErrorBody};
use axum::http::StatusCode;
use uuid::Uuid;

fn build_error(status_code: StatusCode, message: String) -> AppError {
    let error_id = Uuid::new_v4().to_string();

    tracing::debug!(
        "[{}] Request exception. Status code: {}, message: {}",
        error_id,
        status_code,
        message,
    );


    AppError {
        code: status_code,
        error_body: ErrorBody {
            id: error_id,
            message,
        },
    }
}

pub fn build_bad_request(message: String) -> AppError {
    build_error(StatusCode::BAD_REQUEST, message)
}

pub fn build_internal_server_error(message: String) -> AppError {
    build_error(StatusCode::INTERNAL_SERVER_ERROR, message)
}

pub fn build_not_found_error(message: String) -> AppError {
    build_error(StatusCode::NOT_FOUND, message)
}
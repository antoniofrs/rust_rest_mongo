use axum::http::StatusCode;
use uuid::Uuid;
use validator::ValidationErrors;
use crate::error_handler::model::app_error::{AppError, ErrorBody};


fn build_error(status_code: StatusCode, message: String) -> AppError {
    let error_id = Uuid::new_v4().to_string();

    tracing::info!(
        "The request resulted in an exception ({}). Status code: {}, message: {}",
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

fn build_bad_request(message: String) -> AppError {
    build_error(StatusCode::BAD_REQUEST, message)
}

pub fn to_validation_error(validation: ValidationErrors) -> AppError {
    let fields = validation.errors().iter()
        .map(|(key, _)| key.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    build_bad_request(format!("Fields with invalid value: {}", fields))
}

pub fn to_invalid_oid(oid: String) -> AppError {
    build_bad_request(format!("Provided an invalid Id: {}", oid))
}
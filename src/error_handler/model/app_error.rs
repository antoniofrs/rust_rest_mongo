use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub struct AppError {
    pub code: StatusCode,
    pub error_body: ErrorBody,
}

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub message: String,
    pub id: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.code, Json(self.error_body)).into_response()
    }
}





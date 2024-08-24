use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Debug)]
pub struct AppError {
    pub code: StatusCode,
    pub error_body: ErrorBody,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBody {
    pub message: String,
    pub error_id: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.code, Json(self.error_body)).into_response()
    }
}





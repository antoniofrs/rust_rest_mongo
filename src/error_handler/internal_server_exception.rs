use crate::error_handler::base_exception::build_internal_server_error;
use crate::error_handler::model::app_error::AppError;
use mongodb::error::Error;

pub fn database_error(action: &str, database_error: Error) -> AppError {
    tracing::error!("Database error: {}", database_error);

    build_internal_server_error(
        format!("Error while '{}'", action)
    )
}
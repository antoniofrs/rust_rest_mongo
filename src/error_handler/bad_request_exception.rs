use crate::error_handler::base_exception::build_bad_request;
use crate::error_handler::model::app_error::AppError;
use validator::ValidationErrors;


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
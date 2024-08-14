use crate::error_handler::base_exception::build_not_found_error;
use crate::error_handler::model::app_error::AppError;
use mongodb::bson::oid::ObjectId;

pub fn user_not_found_error(user_id: ObjectId) -> AppError {
    build_not_found_error(format!("Cannot find user with id '{}'", user_id.to_string()))
}
use std::sync::Arc;
use axum::extract::State;
use axum::Json;

use crate::dto::user_dto::UserDto;
use crate::error_handler::model::app_error::AppError;
use crate::service::user_service::{UserService, UserServiceTrait};
use crate::support::result_wrapper::ResultWrapper;

pub async fn find_all_users(
    State(user_service): State<Arc<UserService>>,
) -> Result<Json<ResultWrapper<Vec<UserDto>>>, AppError> {
    let users_dto = user_service.find_all().await?;
    Ok(ResultWrapper::json_of(users_dto))
}
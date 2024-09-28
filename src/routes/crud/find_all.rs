use std::sync::Arc;
use axum::extract::State;
use axum::{Extension, Json};
use crate::auth::valid_token::ValidToken;
use crate::dto::user_dto::UserDto;
use crate::error_handler::model::app_error::AppError;
use crate::service::user_service::{UserService, UserServiceTrait};
use crate::support::result_wrapper::ResultWrapper;

pub async fn find_all_users(
    State(user_service): State<Arc<UserService>>,
    Extension(valid_token): Extension<ValidToken>
) -> Result<Json<ResultWrapper<Vec<UserDto>>>, AppError> {
    valid_token.has_permission("read:users").unwrap();
    let users_dto = user_service.find_all().await?;
    Ok(ResultWrapper::json_of(users_dto))
}
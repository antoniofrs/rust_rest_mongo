use std::sync::Arc;
use crate::dto::user_dto::{InsertUserDto, UserDto};
use crate::error_handler::model::app_error::AppError;
use crate::service::user_service::{UserService, UserServiceTrait};
use axum::extract::State;
use axum::{Extension, Json};
use crate::auth::valid_token::ValidToken;

pub async fn create_user(
    State(user_service): State<Arc<UserService>>,
    Json(insert_user_dto): Json<InsertUserDto>,
    Extension(token): Extension<ValidToken>
) -> Result<Json<UserDto>, AppError> {
    token.has_permission("create:users").unwrap();
    let user_dto = user_service.save(insert_user_dto).await?;
    Ok(Json(user_dto))
}
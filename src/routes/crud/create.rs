use crate::dto::user_dto::{InsertUserDto, UserDto};
use crate::error_handler::model::app_error::AppError;
use crate::service::user_service::{UserService, UserServiceTrait};
use axum::extract::State;
use axum::Json;

pub async fn create_user(
    State(user_service): State<UserService>,
    Json(insert_user_dto): Json<InsertUserDto>,
) -> Result<Json<UserDto>, AppError> {
    let user_dto = user_service.save(insert_user_dto).await?;
    Ok(Json(user_dto))
}
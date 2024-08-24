use crate::dto::user_dto::UserDto;
use crate::error_handler::model::app_error::AppError;
use crate::service::user_service::{UserService, UserServiceTrait};
use axum::extract::{Path, State};
use axum::Json;
use mongodb::bson::oid::ObjectId;

pub async fn delete_user(
    State(user_service): State<UserService>,
    Path(id): Path<String>,
) -> Result<Json<UserDto>, AppError> {

    let user_id = ObjectId::parse_str(id).unwrap();
    let user_dto = user_service.delete(user_id).await?;

    Ok(Json(user_dto))
}
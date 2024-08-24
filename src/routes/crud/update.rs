use crate::dto::user_dto::{InsertUserDto, UserDto};
use crate::error_handler::bad_request_exception::to_invalid_oid;
use crate::error_handler::model::app_error::AppError;
use crate::service::user_service::{UserService, UserServiceTrait};
use axum::extract::{Path, State};
use axum::Json;
use mongodb::bson::oid::ObjectId;

pub async fn update_user(
    State(user_service): State<UserService>,
    Path(id): Path<String>,
    Json(insert_user_dto): Json<InsertUserDto>,
) -> Result<Json<UserDto>, AppError> {
    let user_id = ObjectId::parse_str(&id)
        .map_err(|_| { to_invalid_oid(id) })?;

    let user_dto = user_service.update(user_id, insert_user_dto).await?;

    Ok(Json(user_dto))
}
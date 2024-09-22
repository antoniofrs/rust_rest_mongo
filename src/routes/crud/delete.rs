use std::sync::Arc;
use crate::dto::user_dto::UserDto;
use crate::error_handler::model::app_error::AppError;
use crate::service::user_service::{UserService, UserServiceTrait};
use axum::extract::{Path, State};
use axum::{Extension, Json};
use mongodb::bson::oid::ObjectId;
use crate::auth::token::ValidToken;

pub async fn delete_user(
    State(user_service): State<Arc<UserService>>,
    Path(id): Path<String>,
    Extension(valid_token): Extension<ValidToken>
) -> Result<Json<UserDto>, AppError> {
    tracing::info!(valid_token);
    let user_id = ObjectId::parse_str(id).unwrap();
    let user_dto = user_service.delete(user_id).await?;

    Ok(Json(user_dto))
}
use std::sync::Arc;
use crate::dto::user_dto::UserDto;
use crate::error_handler::bad_request_exception::to_invalid_oid;
use crate::error_handler::model::app_error::AppError;
use crate::service::user_service::{UserService, UserServiceTrait};
use axum::extract::{Path, State};
use axum::{Extension, Json};
use mongodb::bson::oid::ObjectId;
use crate::auth::valid_token::ValidToken;

pub async fn find_user_by_id(
    Path(id): Path<String>,
    State(user_service): State<Arc<UserService>>,
    Extension(valid_token): Extension<ValidToken>
) -> Result<Json<UserDto>, AppError> {
    valid_token.has_permission("read:users").unwrap();
    let user_id = ObjectId::parse_str(&id)
        .map_err(|_| { to_invalid_oid(id) })?;

    let user_dto = user_service.find_by_id(user_id).await?;

    Ok(Json(user_dto))
}

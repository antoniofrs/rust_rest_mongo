use crate::dto::user_dto::UserDto;
use crate::error_handler::model::app_error::AppError;
use crate::error_handler::not_found_exception::user_not_found_error;
use crate::model::crud::user::User;
use axum::extract::{Path, State};
use axum::Json;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use crate::error_handler::bad_request_exception::to_invalid_oid;
use crate::repository::user_repository::UserRepository;

pub async fn find_user_by_id(
    Path(id): Path<String>,
    State(db): State<Database>
) ->  Result<Json<UserDto>, AppError> {
    
    let user_id = ObjectId::parse_str(&id)
        .map_err(|_| {to_invalid_oid(id)})?;

    let user = User::by_id(user_id, &db).await?
        .ok_or(user_not_found_error(user_id))?;
    
    let dto = user.to_dto();

    Ok(Json(dto))
}

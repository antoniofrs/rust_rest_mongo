use crate::dto::user_dto::UserDto;
use crate::error_handler::model::app_error::AppError;
use crate::error_handler::not_found_exception::user_not_found_error;
use crate::model::crud::user::User;
use axum::extract::{Path, State};
use axum::Json;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use crate::repository::user_repository::UserRepository;

pub async fn delete_user(
    Path(id): Path<String>,
    State(db): State<Database>
) -> Result<Json<UserDto>, AppError> {
    
    let user_id = ObjectId::parse_str(id).unwrap();
    let user = User::by_id(user_id, &db).await?
        .ok_or(user_not_found_error(user_id))?;
    
    user.delete(&db).await?;

    let dto = user.to_dto();

    Ok(Json(dto))
}
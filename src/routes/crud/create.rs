use crate::dto::user_dto::{InsertUserDto, UserDto};
use crate::error_handler::bad_request_exception::to_validation_error;
use crate::error_handler::model::app_error::AppError;
use crate::repository::user_repository::UserRepository;
use axum::extract::State;
use axum::Json;
use mongodb::Database;
use validator::Validate;

pub async fn create_user(
    State(db): State<Database>,
    Json(insert_user_dto): Json<InsertUserDto>,
) -> Result<Json<UserDto>, AppError> {
    insert_user_dto.validate()
        .map_err(to_validation_error)?;

    let user = insert_user_dto.to_user(None);

    user.save(&db).await?;
    
    let dto = user.to_dto();

    Ok(Json(dto))
}
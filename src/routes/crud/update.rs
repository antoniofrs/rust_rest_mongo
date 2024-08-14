use axum::extract::{Path, State};
use axum::Json;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use validator::Validate;
use crate::dto::user_dto::{InsertUserDto, UserDto};
use crate::error_handler::bad_request_exception::{to_invalid_oid, to_validation_error};
use crate::error_handler::model::app_error::AppError;
use crate::error_handler::not_found_exception::user_not_found_error;
use crate::model::crud::user::User;

pub async fn update_user(
    State(db): State<Database>,
    Path(id): Path<String>,
    Json(insert_user_dto): Json<InsertUserDto>,
) ->  Result<Json<UserDto>, AppError> {

    insert_user_dto.validate()
        .map_err(to_validation_error)?;

    let user_id = ObjectId::parse_str(&id)
        .map_err(|_| {to_invalid_oid(id)})?;

    let user = User::by_id(user_id, &db).await?
        .ok_or(user_not_found_error(user_id))?;

    let user = insert_user_dto.to_user(Option::from(user.id));

    user.update(&db).await?;

    let dto = user.to_dto();

    Ok(Json(dto))
}
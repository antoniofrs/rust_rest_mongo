use axum::extract::State;
use axum::Json;
use mongodb::Database;

use crate::dto::user_dto::UserDto;
use crate::error_handler::model::app_error::AppError;
use crate::model::crud::user::User;
use crate::support::result_wrapper::ResultWrapper;

pub async fn find_all_users(
    State(db): State<Database>,
) -> Result<Json<ResultWrapper<Vec<UserDto>>>, AppError> {
    
    let users = User::find_all(&db).await?;
    
    let dto = users.into_iter()
        .map(|user| {user.to_dto()})
        .collect();

    Ok(ResultWrapper::json_of(dto))
}
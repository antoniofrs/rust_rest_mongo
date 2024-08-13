use axum::extract::State;
use axum::Json;
use mongodb::Database;

use crate::dto::user_dto::UserDto;
use crate::model::crud::user::User;

pub async fn find_all_users(
    State(db): State<Database>,
) -> Json<Vec<UserDto>> {
    
    let users = User::find_all(&db).await
        .unwrap();
    
    let dto = users.into_iter()
        .map(|user| {user.to_dto()})
        .collect();

    Json(dto)
}
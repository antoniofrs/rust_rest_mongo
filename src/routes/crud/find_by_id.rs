use axum::extract::{Path, State};
use axum::Json;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use crate::dto::user_dto::UserDto;
use crate::model::crud::user::User;

pub async fn find_user_by_id(
    Path(id): Path<String>,
    State(db): State<Database>
) -> Json<UserDto> {
    
    let user_id = ObjectId::parse_str(id).unwrap();
    
    let user = User::by_id(user_id, &db).await
        .unwrap();
    
    let dto = user.to_dto();

    Json(dto)
}
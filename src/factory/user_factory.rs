use mongodb::bson::oid::ObjectId;
use crate::dto::user_dto::{InsertUserDto, UserDto};
use crate::model::crud::user::User;


impl User {
    pub fn to_dto(self) -> UserDto {
        UserDto {
            id: self.id.to_string(),
            name: self.name,
            surname: self.surname,
            email: self.email,
        }
    }
}

impl InsertUserDto {
    pub fn to_user(self, id: Option<ObjectId>) -> User {
        User {
            id: id.unwrap_or(ObjectId::new()),
            email: self.email,
            name: self.name,
            surname: self.surname,
        }
    }
}
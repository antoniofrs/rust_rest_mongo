use mongodb::bson::oid::ObjectId;
use crate::dto::user_dto::InsertUserDto;
use crate::model::crud::user::User;

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
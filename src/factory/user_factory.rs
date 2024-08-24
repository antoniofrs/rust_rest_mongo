use crate::dto::user_dto::UserDto;
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

#[cfg(test)]
mod tests {
    use crate::dto::user_dto::UserDto;
    use crate::model::crud::user::User;
    use mongodb::bson::oid::ObjectId;

    #[test]
    fn converts_user_to_user_dto_correctly() {
        let id = ObjectId::new();

        let user = User {
            id: id.clone(),
            name: String::from("name"),
            surname: String::from("surname"),
            email: String::from("email@example.com"),
        };

        let expected_dto = UserDto {
            id: id.to_string(),
            name: String::from("name"),
            surname: String::from("surname"),
            email: String::from("email@example.com"),
        };

        let user_dto = user.to_dto();

        assert_eq!(user_dto, expected_dto);
    }
}
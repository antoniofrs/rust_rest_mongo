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


#[cfg(test)]
mod tests {
    use mongodb::bson::oid::ObjectId;
    use crate::dto::user_dto::InsertUserDto;

    #[test]
    fn test_to_user_with_id() {
        let dto = InsertUserDto {
            email: "test@example.com".to_string(),
            name: "name".to_string(),
            surname: "surname".to_string(),
        };

        let id = ObjectId::new();
        let user = dto.clone().to_user(Some(id.clone()));

        assert_eq!(user.id, id);
        assert_eq!(user.email, dto.email);
        assert_eq!(user.name, dto.name);
        assert_eq!(user.surname, dto.surname);
    }

    #[test]
    fn test_to_user_without_id() {
        let dto = InsertUserDto {
            email: "test@example.com".to_string(),
            name: "name".to_string(),
            surname: "surname".to_string(),
        };

        let user = dto.clone().to_user(None);

        assert_eq!(user.email, dto.email);
        assert_eq!(user.name, dto.name);
        assert_eq!(user.surname, dto.surname);
    }
}

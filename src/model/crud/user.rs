use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub email: String,
    pub name: String,
    pub surname: String,
}


#[cfg(test)]
mod tests {
    use mongodb::bson::oid::ObjectId;
    use crate::dto::user_dto::InsertUserDto;
    use crate::model::crud::user::User;

    #[test]
    fn converts_insert_user_dto_to_user_with_provided_id() {
        let provided_id = ObjectId::new();

        let insert_user_dto = InsertUserDto {
            email: String::from("email@example.com"),
            name: String::from("name"),
            surname: String::from("surname"),
        };

        let expected_user = User {
            id: provided_id.clone(),
            email: String::from("email@example.com"),
            name: String::from("name"),
            surname: String::from("surname"),
        };

        let user = insert_user_dto.to_user(Some(provided_id));

        assert_eq!(user, expected_user);
    }

    #[test]
    fn converts_insert_user_dto_to_user_with_new_id_when_no_id_provided() {
        let insert_user_dto = InsertUserDto {
            email: String::from("email@example.com"),
            name: String::from("name"),
            surname: String::from("surname"),
        };

        let user = insert_user_dto.to_user(None);

        assert_eq!(user.email, "email@example.com");
        assert_eq!(user.name, "name");
        assert_eq!(user.surname, "surname");
        assert!(!user.id.to_string().is_empty());
    }
}
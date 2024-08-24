use app::dto::user_dto::InsertUserDto;

pub fn test_insert_user_sto() -> InsertUserDto {
    InsertUserDto {
        name: String::from("name"),
        surname: String::from("surname"),
        email: String::from("email@email.com"),
    }
}
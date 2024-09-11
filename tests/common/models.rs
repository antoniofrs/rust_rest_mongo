use app::dto::user_dto::InsertUserDto;

pub fn insert_user_sto() -> InsertUserDto {
    InsertUserDto {
        name: String::from("name"),
        surname: String::from("surname"),
        email: String::from("email@email.com"),
    }
}

pub fn insert_user_sto_invalid_mail() -> InsertUserDto {
    InsertUserDto {
        name: String::from("name"),
        surname: String::from("surname"),
        email: String::from("invalid-mail.com"),
    }
}
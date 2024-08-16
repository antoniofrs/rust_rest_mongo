use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, PartialEq)]
pub struct UserDto {
    pub id: String,
    pub email: String,
    pub name: String,
    pub surname: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct InsertUserDto {
    #[validate(email)]
    pub email: String,
    pub name: String,
    pub surname: String,
}
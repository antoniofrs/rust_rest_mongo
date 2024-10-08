use crate::dto::user_dto::{InsertUserDto, UserDto};
use crate::error_handler::bad_request_exception::to_validation_error;
use crate::error_handler::model::app_error::AppError;
use crate::error_handler::not_found_exception::user_not_found_error;
use crate::repository::user_repository::UserRepositoryTrait;
use async_trait::async_trait;
use axum::extract::FromRef;
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use validator::Validate;
use crate::support::sqs_listener::SqsListener;

#[derive(Clone, FromRef)]
pub struct UserService {
    pub user_repo: Arc<dyn UserRepositoryTrait + Send + Sync>,
}

impl UserService {
    pub fn init(user_repo: Arc<dyn UserRepositoryTrait + Send + Sync>) -> UserService {
        UserService { user_repo }
    }
}

#[async_trait]
pub trait UserServiceTrait {
    async fn find_all(&self) -> Result<Vec<UserDto>, AppError>;
    async fn save(&self, insert_user_dto: InsertUserDto) -> Result<UserDto, AppError>;
    async fn update(&self, id: ObjectId, insert_user_dto: InsertUserDto) -> Result<UserDto, AppError>;
    async fn delete(&self, id: ObjectId) -> Result<UserDto, AppError>;
    async fn find_by_id(&self, id: ObjectId) -> Result<UserDto, AppError>;
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn find_all(&self) -> Result<Vec<UserDto>, AppError> {
        let users = self.user_repo.find_all().await?;

        let dto = users.into_iter()
            .map(|user| { user.to_dto() })
            .collect();

        Ok(dto)
    }

    async fn save(&self, insert_user_dto: InsertUserDto) -> Result<UserDto, AppError> {
        insert_user_dto.validate()
            .map_err(to_validation_error)?;

        let user = insert_user_dto.to_user(None);
        self.user_repo.save(&user).await?;

        Ok(user.to_dto())
    }

    async fn update(&self, id: ObjectId, insert_user_dto: InsertUserDto) -> Result<UserDto, AppError> {
        insert_user_dto.validate()
            .map_err(to_validation_error)?;

        let user = insert_user_dto.to_user(Option::from(id));

        self.user_repo.update(&user).await?;
        Ok(user.to_dto())
    }

    async fn delete(&self, id: ObjectId) -> Result<UserDto, AppError> {
        let user = self.user_repo.find_by_id(id).await?
            .ok_or_else(|| { user_not_found_error(id) })?;
        self.user_repo.delete(&user).await?;
        Ok(user.to_dto())
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<UserDto, AppError> {
        let user = self.user_repo.find_by_id(id).await?
            .ok_or_else(|| { user_not_found_error(id) })?;
        Ok(user.to_dto())
    }
}
#[async_trait]
impl SqsListener for UserService {
    async fn on_message_received(&self, message: String) -> () {
        println!("{}", message);
    }
}
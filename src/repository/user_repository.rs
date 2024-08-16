use crate::error_handler::internal_server_exception::database_error;
use crate::error_handler::model::app_error::AppError;
use crate::model::crud::user::User;
use async_trait::async_trait;
use futures::TryStreamExt;
use mockall::automock;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};

#[derive(Clone)]
pub struct UserRepository {
    users: Collection<User>
}

#[async_trait]
pub trait UserRepositoryTrait {

    fn init(db: Database) -> UserRepository;

    async fn find_all(&self) -> Result<Vec<User>, AppError>;
    async fn save(&self, user: &User) -> Result<(), AppError>;
    async fn update(&self, user: &User) -> Result<(), AppError>;
    async fn delete(&self, user: &User) -> Result<(), AppError>;
    async fn find_by_id(&self, id: ObjectId) -> Result<Option<User>, AppError>;
}


#[async_trait]
impl UserRepositoryTrait for UserRepository {

    fn init(db: Database) -> UserRepository {
        let users = db.collection("users");
        UserRepository { users }
    }


    async fn find_all(&self) -> Result<Vec<User>, AppError> {
        let cursor = self.users
            .find(doc! {}).await
            .map_err(|e| { database_error("finding all users", e)})?;

        let users = cursor.try_collect().await
            .map_err(|e| { database_error("collecting users", e)})?;

        Ok(users)
    }

    async fn save(&self, user: &User) -> Result<(), AppError> {
        self.users.insert_one(user).await
            .map_err(|e| { database_error("saving user", e) })?;
        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), AppError> {
        self.users.update_one(
                doc! { "_id": user.id },
                doc! { "$set":
                    {
                    "email": &user.email,
                    "name": &user.name,
                    "surname": &user.surname
                }
            },
            ).await
            .map_err(|e| { database_error("updating user", e) })?;
        Ok(())
    }

    async fn delete(&self, user: &User) -> Result<(), AppError> {
        self.users.delete_one(doc! { "_id": &user.id }).await
            .map_err(|e| { database_error("deleting user", e) })?;
        Ok(())
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<Option<User>, AppError> {
        let user = self.users
            .find_one(doc! { "_id": id }).await
            .map_err(|e| { database_error("find user by id", e) })?;

        Ok(user)
    }

}
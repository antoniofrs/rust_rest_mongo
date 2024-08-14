use futures::TryStreamExt;
use crate::error_handler::internal_server_exception::database_error;
use crate::error_handler::model::app_error::AppError;
use crate::model::crud::user::User;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};

impl User {
    pub async fn find_all(database: &Database) -> Result<Vec<User>, AppError> {
        let cursor = User::get_collection(database)
            .find(doc! {}).await
            .map_err(|e| { database_error("finding all users", e)})?;

        let users = cursor.try_collect().await
            .map_err(|e| { database_error("collecting users", e)})?;

        Ok(users)
    }

    pub async fn save(&self, database: &Database) -> Result<(), AppError> {
        User::get_collection(database)
            .insert_one(self).await
            .map_err(|e| { database_error("saving user", e) })?;
        Ok(())
    }

    pub async fn update(&self, database: &Database) -> Result<(), AppError> {
        User::get_collection(database)
            .update_one(
                doc! { "_id": &self.id },
                doc! { "$set":
                    {
                    "email": &self.email,
                    "name": &self.name,
                    "surname": &self.surname
                }
            },
            ).await
            .map_err(|e| { database_error("updating user", e) })?;
        Ok(())
    }

    pub async fn delete(&self, database: &Database) -> Result<(), AppError> {
        User::get_collection(database)
            .delete_one(doc! { "_id": &self.id }).await
            .map_err(|e| { database_error("deleting user", e) })?;
        Ok(())
    }

    pub async fn by_id(id: ObjectId, database: &Database) -> Result<Option<User>, AppError> {
        let user = User::get_collection(database)
            .find_one(doc! { "_id": id }).await
            .map_err(|e| { database_error("find user by id", e) })?;

        Ok(user)
    }

    fn get_collection(database: &Database) -> Collection<User> {
        database.collection::<User>("user")
    }
}
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use futures::TryStreamExt;
use crate::error_handler::internal_server_exception::database_error;
use crate::error_handler::model::app_error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub email: String,
    pub name: String,
    pub surname: String,
}

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
            .map_err(|e| { database_error("User", e) })?;
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
            .map_err(|e| { database_error("User", e) })?;

        Ok(user)
    }

    fn get_collection(database: &Database) -> Collection<User> {
        database.collection::<User>("user")
    }
}
use std::fmt::Error;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use futures::TryStreamExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub email: String,
    pub name: String,
    pub surname: String,
}

impl User {
    pub async fn find_all(database: &Database) -> Result<Vec<User>, Error> {
        let cursor = User::get_collection(database)
            .find(doc! {}).await
            .unwrap();

        let users = cursor.try_collect().await
            .unwrap();

        Ok(users)
    }

    pub async fn save(&self, database: &Database) -> Result<(), Error> {
        User::get_collection(database)
            .insert_one(self).await
            .unwrap();
        Ok(())
    }

    pub async fn update(&self, database: &Database) -> Result<(), Error> {
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
            .unwrap();
        Ok(())
    }

    pub async fn delete(&self, database: &Database) -> Result<(), Error> {
        User::get_collection(database)
            .delete_one(doc! { "_id": &self.id }).await
            .unwrap();
        Ok(())
    }

    pub async fn by_id(id: ObjectId, database: &Database) -> Result<User, Error> {
        let user = User::get_collection(database)
            .find_one(doc! { "_id": id }).await
            .unwrap();
        Ok(user.unwrap())
    }

    fn get_collection(database: &Database) -> Collection<User> {
        database.collection::<User>("user")
    }
}
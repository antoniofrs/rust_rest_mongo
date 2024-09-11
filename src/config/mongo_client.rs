use mongodb::bson::doc;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::{Client, Database};
use std::env;

pub struct MongoConfig {
    pub uri: String,
    pub database_name: String,
}

impl Default for MongoConfig {
    fn default() -> Self {
        MongoConfig {
            uri: env::var("MONGO_URI").unwrap(),
            database_name: env::var("DB_NAME").unwrap()
        }
    }
}

pub async fn get_database(mongo_config: MongoConfig) -> Database {
    let mut client_options = ClientOptions::parse(mongo_config.uri).await.unwrap();

    let server_api = ServerApi::builder()
        .version(ServerApiVersion::V1)
        .build();

    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options).unwrap();

    client.database("admin").run_command(doc! { "ping": 1 }).await
        .expect("MongoDb connection failed");

    tracing::info!("Deployment pinged. Successfully connected to MongoDB");

    client.database(mongo_config.database_name.leak())
}
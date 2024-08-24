use std::env;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::{Client, Database};
use mongodb::bson::doc;

pub struct MongoConfig {
    pub uri: String,
    pub database_name: String
}

pub async fn get_database_from_env() -> Database {

    let uri = env::var("MONGO_URI")
        .unwrap_or("mongodb://root:pass@localhost:27017/?authSource=admin&w=majority".to_owned());

    let database_name = env::var("DB_NAME")
        .unwrap_or("rust-rest".to_owned());

    let config = MongoConfig{ uri, database_name };

    get_database(config).await


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
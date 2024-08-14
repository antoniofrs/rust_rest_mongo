use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::{Client, Database};
use mongodb::bson::doc;

pub async fn mongo_client(uri: String, database: String) -> Database {
    let mut client_options = ClientOptions::parse(uri).await.unwrap();

    let server_api = ServerApi::builder()
        .version(ServerApiVersion::V1)
        .build();

    client_options.server_api = Some(server_api);
    
    let client = Client::with_options(client_options).unwrap();

    client.database("admin").run_command(doc! { "ping": 1 }).await
        .expect("MongoDb connection failed");

    tracing::info!("Deployment pinged. Successfully connected to MongoDB");

    client.database(database.leak())
}
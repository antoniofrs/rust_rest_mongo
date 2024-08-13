use mongodb::{Client, Database};
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};

pub async fn mongo_client(uri: String, database: String) -> Database {
    let mut client_options = ClientOptions::parse(uri).await.unwrap();

    let server_api = ServerApi::builder()
        .version(ServerApiVersion::V1)
        .build();

    client_options.server_api = Some(server_api);
    
    let client = Client::with_options(client_options).unwrap();
    
    tracing::info!("successfully connected to MongoDB");
    
    client.database(database.leak())
}